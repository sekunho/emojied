{ pkgs, unstablepkgs }:

pkgs.mkShell {
  buildInputs = [
    # Back-end
    pkgs.rustc
    pkgs.cargo
    unstablepkgs.cargo-flamegraph

    # Front-end
    unstablepkgs.nodePackages.typescript
    unstablepkgs.nodePackages.typescript-language-server
    unstablepkgs.nodePackages.tailwindcss
    unstablepkgs.esbuild

    pkgs.openssl
    pkgs.pkg-config

    # Database
    pkgs.sqitchPg
    pkgs.perl534Packages.TAPParserSourceHandlerpgTAP

    # Dev tools
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.rustfmt
    pkgs.cargo-watch
    pkgs.flyctl
    pkgs.zip
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  APP__STATIC_ASSETS = "";
}
