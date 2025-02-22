{ pkgs, ... }: rec {
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  APP__SERVER_PORT = 3000;
  APP__SERVER__STATIC_ASSETS = "./public";
  APP__DATABASE__NAME = "emojied_local.db";

  buildInputs = with pkgs; [
    nodePackages.tailwindcss
    esbuild
    openssl
    pkg-config
    cargo-watch

    sqitchSqlite
    sqlite

    git

    nil
    nixpkgs-fmt
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
    libiconv
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.CoreServices
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ]);
}
