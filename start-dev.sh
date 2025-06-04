#!/bin/bash
set -e

# Function to clean up on exit
cleanup() {
  echo "Shutting down services..."
  docker compose down
  if [ -n "$BACKEND_PID" ]; then
    echo "Stopping backend..."
    kill $BACKEND_PID 2>/dev/null || true
  fi
  exit
}

# Set up cleanup trap
trap cleanup EXIT INT TERM

# Start TimescaleDB
echo "Starting TimescaleDB..."
docker compose up -d

# Wait for database to be ready
echo "Waiting for database to be ready..."
for i in {1..30}; do
  if docker compose exec timescaledb pg_isready -U ownhealth -d ownhealth 2>/dev/null; then
    echo "Database is ready!"
    break
  fi
  echo "Waiting for database... ($i/30)"
  sleep 1
  if [ $i -eq 30 ]; then
    echo "Database failed to start within 30 seconds"
    exit 1
  fi
done

# Start backend using nix
echo "Starting backend..."
# Run nix in foreground to show compilation output
nix run '.#backend' &
BACKEND_PID=$!

# Wait for backend to be ready (check if it's listening on port 3000)
until nc -z localhost 3000 2>/dev/null; do
  # Check if the backend process is still running
  if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "Backend process terminated unexpectedly"
    exit 1
  fi
  sleep 1
done

echo "All services started!"
echo "- TimescaleDB running at localhost:5432"
echo "- Backend running at http://localhost:3000"

echo "Press Ctrl+C to stop all services"
# Keep script running until interrupted
wait