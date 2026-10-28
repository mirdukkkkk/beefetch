{ lib, rustPlatform }:

let
  cargoToml = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  inherit (cargoToml.package) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    inherit (cargoToml.package) description;
    homepage = cargoToml.package.repository;
    license = lib.licenses.gpl3Only;
    mainProgram = "beefetch";
    platforms = lib.platforms.linux;
  };
}
