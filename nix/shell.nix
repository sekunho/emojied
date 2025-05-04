{ pkgs, ... }: rec {
  env = {
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    PG__DBNAME = "emojied_development";
    PG__HOST = "127.0.0.1";
    PG__USER = "emojied";
    PG__PASSWORD = "emojied";
    PG__PORT = 5433;
    APP__STATIC_ASSETS = "./public";
  };

  packages = with pkgs; [
    nodePackages.tailwindcss
    esbuild
    openssl
    pkg-config

    sqitchPg
    perl540Packages.TAPParserSourceHandlerpgTAP
  ];

  languages = {
    rust.enable = true;
    typescript.enable = true;
  };

  services.postgres = {
    enable = true;
    port = env.PG__PORT;
    package = pkgs.postgresql_15;
    listen_addresses = env.PG__HOST;
    initialDatabases = [ { name = env.PG__DBNAME; } ];

    initialScript = ''
      CREATE USER ${env.PG__USER} SUPERUSER PASSWORD '${env.PG__PASSWORD}';
    '';
  };
}

/* pkgs.mkShell { */
/*   buildInputs = with pkgs; [ */
/*     # Back-end */
/*     rustc */
/*     cargo */
/*     cargo-flamegraph */

/*     # Front-end */
/*     nodePackages.typescript */
/*     nodePackages.typescript-language-server */
/*     nodePackages.tailwindcss */
/*     esbuild */

/*     openssl */
/*     pkg-config */

/*     # Database */
/*     sqitchPg */
/*     perl534Packages.TAPParserSourceHandlerpgTAP */

/*     # Dev tools */
/*     rust-analyzer */
/*     clippy */
/*     rustfmt */
/*     cargo-watch */
/*     flyctl */
/*     zip */
/*   ]; */


/* } */
