{
  description = "beefetch - 🐝";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (
          system: f nixpkgs.legacyPackages.${system}
        );
    in
    {
      overlays.default = final: _: { beefetch = final.callPackage ./package.nix { }; };

      packages = forAllSystems (pkgs: {
        beefetch = pkgs.callPackage ./package.nix { };
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.beefetch;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          # Listed explicitly instead of inputsFrom so the shell still works
          # when the package itself fails to evaluate (e.g. missing Cargo.lock).
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
            hyperfine
          ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
