{
  description = "Craft flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let name = "craft";
          src = ./.;
          overlays = [
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          rust = pkgs.rust-bin.nightly.latest.default;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (rust.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ];
        };
      }
    );
}
