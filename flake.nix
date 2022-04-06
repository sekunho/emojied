{
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixos-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };
outputs = { self, nixpkgs, nixos-unstable, flake-utils }: flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
          unstablepkgs = nixos-unstable.legacyPackages.${system};
      in
      {
        # checks = {
        #   pre-commit-check = pre-commit-hooks.lib.${system}.run {
        #     src = ./.;
        #     hooks = {
        #       cargo-check.enable = true;
        #       clippy.enable = true;
        #       rustfmt.enable = true;
        #     };
        #   };
        # };
        devShells.ci = pkgs.mkShell rec {
          buildInputs = [
            pkgs.sqitchPg
            pkgs.perl534Packages.TAPParserSourceHandlerpgTAP
          ];
        };

        devShell = pkgs.mkShell {
          # inherit (self.checks.${system}.pre-commit-check) shellHook;

          buildInputs = with pkgs; [
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
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      });
}
