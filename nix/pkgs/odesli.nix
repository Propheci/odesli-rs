{
  lib,
  makeRustPlatform,
  nix-filter,
  openssl,
  pkg-config,
  toolchain,
}: let
  pkgName = "odesli";

  localSrc = nix-filter {
    name = pkgName;
    root = ../..;
    include = [
      ../../Cargo.toml
      ../../Cargo.lock
      ../../bin
      ../../lib
    ];
  };
in
  (makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  })
  .buildRustPackage
  {
    name = pkgName;
    version = "0.1.0";

    nativeBuildInputs = [pkg-config];
    buildInputs = [openssl.dev];

    PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";

    src = localSrc;
    cargoLock.lockFile = ../../Cargo.lock;

    meta = with lib; {
      description = "Nix package for ${pkgName}";
      homepage = "Add link here";
      license = licenses.mit;
      mainProgram = pkgName;
    };
  }
