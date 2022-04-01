{
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixos-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixpkgs, nixos-unstable, flake-utils, naersk }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
          unstablepkgs = nixos-unstable.legacyPackages.${system};
          naersk-lib = naersk.lib."${system}".override {
            cargo = pkgs.cargo;
            rustc = pkgs.rustc;
          };
      in
      rec {
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

        packages.emojied = naersk-lib.buildPackage {
          pname = "emojied";
          root = ./.;
        };

        defaultPackage = packages.emojied;

        apps.emojied = flake-utils.lib.mkApp {
          drv = packages.emojied;
        };

        defaultApp = apps.emojied;

        devShell = pkgs.mkShell {
          # inherit (self.checks.${system}.pre-commit-check) shellHook;

          nativeBuildInputs = with pkgs; [
            # Back-end
            pkgs.rustc
            pkgs.cargo

            # Front-end
            unstablepkgs.nodePackages.typescript
            unstablepkgs.nodePackages.typescript-language-server
            unstablepkgs.nodePackages.tailwindcss
            unstablepkgs.esbuild

            # Database
            pkgs.sqitchPg
            pkgs.perl534Packages.TAPParserSourceHandlerpgTAP

            # Dev tools
            pkgs.rust-analyzer
            pkgs.clippy
            pkgs.rustfmt
            pkgs.cargo-watch
          ];
        };
      });
}
