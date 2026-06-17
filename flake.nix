{
  description = "beefetch - 🐝";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        packages.default = pkgs.buildGoModule {
          pname = "beefetch";
          version = "2.0.1";

          src = ./.;

          vendorHash = null;

          env = {
            CGO_ENABLED = 0;
          };
          ldflags = [ "-s" "-w" ];

          meta = {
            description = "Neofetch clone written in Go with the ability to display 🐝";
            homepage = "https://github.com/mirdukkkkk/beefetch";
            license = pkgs.lib.licenses.gpl3Only;
            mainProgram = "beefetch";
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.go
          ];
        };
      }
    );
}
