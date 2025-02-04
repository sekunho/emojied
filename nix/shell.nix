{ pkgs, ... }: rec {
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  PG__DBNAME = "emojied_development";
  PG__HOST = "127.0.0.1";
  PG__USER = "emojied";
  PG__PASSWORD = "emojied";
  PG__PORT = 5433;
  APP__STATIC_ASSETS = "./public";

  buildInputs = with pkgs; [
    nodePackages.tailwindcss
    esbuild
    openssl
    pkg-config

    nil
    nixpkgs-fmt
  ];
}
