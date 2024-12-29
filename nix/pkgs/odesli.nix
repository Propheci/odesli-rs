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

  packageDescription = builtins.fromTOML (
    builtins.readFile ../../bin/Cargo.toml
  );
in
  (makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  })
  .buildRustPackage
  {
    name = pkgName;
    version = packageDescription.package.version;

    nativeBuildInputs = [pkg-config];
    buildInputs = [openssl.dev];

    PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";

    src = localSrc;
    cargoLock.lockFile = ../../Cargo.lock;

    meta = with lib; {
      description = "Nix package for ${pkgName}";
      homepage = "https://github.com/Propheci/odesli-rs";
      license = licenses.mit;
      mainProgram = pkgName;
    };
  }
