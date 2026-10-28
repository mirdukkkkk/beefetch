{
  lib,
  stdenv,
  rustPlatform,
  installShellFiles,
}:

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
      ./docs/beefetch.1
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installManPage docs/beefetch.1
  ''
  + lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    installShellCompletion --cmd beefetch \
      --bash <($out/bin/beefetch --bpaf-complete-style-bash) \
      --fish <($out/bin/beefetch --bpaf-complete-style-fish) \
      --zsh <($out/bin/beefetch --bpaf-complete-style-zsh)
  '';

  meta = {
    inherit (cargoToml.package) description;
    homepage = cargoToml.package.repository;
    license = lib.licenses.gpl3Only;
    mainProgram = "beefetch";
    platforms = lib.platforms.linux;
  };
}
