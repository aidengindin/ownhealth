{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk {};
        nodejs = pkgs.nodejs_20;
      in rec {
        packages = {
          backend = naersk'.buildPackage {
            src = ./backend;
          };

          frontend = pkgs.buildNpmPackage {
            pname = "ownhealth-frontend";
            version = "0.1.0";

            src = ./frontend;
            npmDepsHash = "sha256-0seyKqI9ZgnbENYTGEeokbFK65NY0uCCHoAuSOiiVbo=";

            installPhase = ''
              runHook preInstall
              cp -r dist/* $out/
              runHook postInstall
            '';

            nodejs = nodejs;
            nativeBuildInputs = [ nodejs ];
          };
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            lld

            nodejs
          ];
          shellHook = ''
            echo "🦀 $(rustc --version)"
            echo "📦 Node $(node --version) (npm $(npm --version))"
          '';
        };
      }
    );
}

