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

        dbHost = mkOption {
          default = "localhost";
          type = with types; str;
          description = "Host of database server";
        };

        dbName = mkOption {
          default = "emojied_db";
          type = with types; uniq str;
          description = "Database name for emojied";
        };

        dbUser = mkOption {
          default = "postgres";
          type = with types; uniq str;
          description = "Database user";
        };

        dbPort = mkOption {
          default = "5432";
          type = with types; str;
          description = "Port number of database server";
        };

        dbPasswordFile = mkOption {
          type = with types; uniq str;
          description = "Path to DB password file";
        };

        dbCACertFile = mkOption {
          default = "";
          type = with types; uniq str;
          description = "Path to DB CA certificate";
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

        environment = mkMerge [
          {
            APP__PORT = "${cfg.port}";
            PG__DBNAME = "${cfg.dbName}";
            PG__HOST = "${cfg.dbHost}";
            PG__USER = "${cfg.dbUser}";
            PG__PORT = "${cfg.dbPort}";
            PG__PASSWORD_FILE = "${cfg.dbPasswordFile}";
          }

          (mkIf ("${cfg.dbCACertFile}" != "") {
            PG__CA_CERT = "${cfg.dbCACertFile}";
          })
        ];

        serviceConfig = {
          Type = "simple";
          ExecStart = "${emojied}/bin/emojied";
        };
      };

      environment.systemPackages = [ emojied ];
    };
  }
