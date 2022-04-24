{ config, pkgs, lib, emojied, ... }:
  let
    cfg = config.services.emojied;
  in with lib; {
    options = {
      services.emojied = {
        enable = mkOption {
          default = false;
          type = with types; bool;
          description = "Start the emojied server for a user";
        };

        port = mkOption {
          default = "3000";
          type = with types; str;
          description = "Port number emojied will run on";
        };

        db_host = mkOption {
          default = "localhost";
          type = with types; str;
          description = "Host of database server";
        };

        db_name = mkOption {
          default = "emojied_db";
          type = with types; uniq str;
          description = "Database name for emojied";
        };

        db_user = mkOption {
          default = "postgres";
          type = with types; uniq str;
          description = "Database user";
        };

        db_password = mkOption {
          default = "";
          type = with types; uniq str;
          description = "Database user's password";
        };

        db_port = mkOption {
          default = "5432";
          type = with types; str;
          description = "Port number of database server";
        };
      };
    };

    config = mkIf cfg.enable {
      # services.postgresql = {
      #   enable = true;
      #   extraPlugins = with pkgs.postgresql14Packages; [ pgtap ];
      #   package = pkgs.postgresql_14;

      #   # FIXME: Should change this one lol
      #   authentication = pkgs.lib.mkOverride 14 ''
      #     local all all trust
      #     host all all ::1/128 trust
      #     host all all localhost trust
      #   '';
      # };

      systemd.services.emojied = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Start the emojied server";

        environment = {
          APP__PORT = "${cfg.port}";
          PG__DBNAME = "${cfg.db_name}";
          PG__HOST = "${cfg.db_host}";
          PG__USER = "${cfg.db_user}";
          PG__PORT = "${cfg.db_port}";
          PG__PASSWORD = "${cfg.db_password}";

          /* inherit optionalAttrs (cfg.db_password != "") */
            /* { PG__PASSWORD = "${cfg.db_password}"; }; */
        };

        serviceConfig = {
          Type = "simple";
          ExecStart = "${emojied}/bin/emojied";
        };
      };

      environment.systemPackages = [ emojied ];
    };
  }
