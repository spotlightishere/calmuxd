{ self, ... }:
{ pkgs, config, lib, ... }:
let
  inherit (lib) mkEnableOption mkIf mkOption optionalString types;
  cfg = config.services.calmuxd;

  # Used for global emoji configuration.
  json = pkgs.formats.json { };
  configFile = json.generate "config.json" cfg.settings;
in
{
  options.services.calmuxd = {
    enable = mkEnableOption (lib.mdDoc "calmuxd");

    settings = lib.mkOption {
      type = lib.types.nullOr json.type;
      default = null;
      description = ''
        The content of /etc/calmuxd/config.json.
        Refer to https://github.com/spotlightishere/calmuxd/blob/main/config.example.json.
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services.calmuxd = {
      description = "Simple calendar feed muxing agent";
      wantedBy = [ "multi-user.target" ];
      serviceConfig.ExecStart = "${self.packages.${pkgs.system}.calmuxd}/bin/calmuxd ${configFile}";
    };
  };
}
