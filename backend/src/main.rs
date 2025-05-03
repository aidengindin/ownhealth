pub mod domain;

use axum::{
    routing::{get, post},
    response::IntoResponse,
    http::StatusCode,
    extract::{Path, Query},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tracing::debug;
use tracing_subscriber;

use domain::{
    DataPoint,
    DataPointSeries,
    HeartRateType,
    WeightType,
    VO2MaxType,
    HydrationType,
    SleepStageType,
    SleepDurationType,
};

#[derive(Deserialize, Debug)]
struct MetricParams {
    metric_name: String,
}

#[derive(Deserialize, Debug)]
struct DateRangeParams {
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum AnyDataPointSeries {
    HeartRate(DataPointSeries<HeartRateType>),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/metric/{metric_name}", get(metric_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn metric_handler(
    Path(params): Path<MetricParams>,
    Query(date_range): Query<DateRangeParams>,
) -> impl IntoResponse {
    let metric_name = params.metric_name;
    debug!("Date range: from={:?}, to={:?}", date_range.from, date_range.to);

    let ts = Utc::now();
    let result: Result<AnyDataPointSeries, StatusCode> = match metric_name.as_str() {
        "heart_rate" => {
            let point1 = DataPoint::<HeartRateType>::new(50, ts);
            let point2 = DataPoint::<HeartRateType>::new(52, ts);
            let series = DataPointSeries::<HeartRateType>::new(vec![point1, point2]);
            Ok(AnyDataPointSeries::HeartRate(series))
        }
        _ => {
            Err(StatusCode::NOT_FOUND)
        }
    };

    match result {
        Ok(data_point) => Json(data_point).into_response(),
        Err(status_code) => status_code.into_response(),
    }
}

