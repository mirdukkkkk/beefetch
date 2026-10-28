{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.beefetch;

  toValue = v: if lib.isList v then lib.concatStringsSep ", " v else lib.boolToString v;
  settings = lib.filterAttrs (_: v: v != null) cfg.settings;
in
{
  options.programs.beefetch = {
    enable = lib.mkEnableOption "beefetch, a neofetch clone with 🐝";

    package = lib.mkOption {
      type = lib.types.package;
      # Built from the user's pkgs, so no second nixpkgs instance is pulled in.
      default = pkgs.callPackage ./package.nix { };
      defaultText = lib.literalExpression "pkgs.callPackage beefetch/package.nix { }";
      description = "The beefetch package to install.";
    };

    settings = {
      fields = lib.mkOption {
        type = lib.types.nullOr (
          lib.types.listOf (
            lib.types.enum [
              "os"
              "kernel"
              "uptime"
              "ram"
              "wm"
              "server"
            ]
          )
        );
        default = null;
        example = [
          "os"
          "kernel"
          "ram"
        ];
        description = "Fields to show, in order. `null` keeps the built-in default (all fields).";
      };

      color = lib.mkOption {
        type = lib.types.nullOr lib.types.bool;
        default = null;
        example = false;
        description = "Whether to use ANSI colors. `null` keeps the built-in default (`true`).";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."beefetch/config" = lib.mkIf (settings != { }) {
      text = lib.concatStrings (lib.mapAttrsToList (k: v: "${k} = ${toValue v}\n") settings);
    };
  };
}
