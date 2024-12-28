{
  description = "Flake for various things related to odesli-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    nix-filter,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        rustToolchain = fenix.packages."${system}".stable.toolchain;
      in
        with pkgs; {
          devShells.default = mkShell {
            name = "odesli-rs";

            buildInputs = [
              clang
              rustToolchain
              protobuf
              taplo
              pkg-config

              openssl.dev
              openssl
            ];
          };

          packages = rec {
            odesli = (
              pkgs.callPackage ./pkgs/odesli.nix {
                inherit nix-filter;
                toolchain = rustToolchain;
              }
            );
            default = odesli;
          };
        }
    );
}
