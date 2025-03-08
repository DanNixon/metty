{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
      in {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            cargo
            rustc

            # Code analysis tools
            rust-analyzer
            clippy

            # Code formatting tools
            treefmt
            alejandra
            mdl
            rustfmt
          ];
        };
      }
    );
}
