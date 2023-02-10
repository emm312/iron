{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rocket = with pkgs;
          import ./default.nix {
            inherit rustPlatform;
            inherit lib;
          };
      in
        with pkgs; {
          defaultApp = rocket;
          defaultPackage = rocket;
          devShell = mkShell {
            buildInputs = [
              rust-analyzer
              cargo-nextest
              rust-bin.beta.latest.default
              lldb
              evcxr
            ];
          };
        }
    );
}
