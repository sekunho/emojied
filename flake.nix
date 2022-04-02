{
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixos-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:sekunho/crate2nix/sekunho/dedup-sources";
      flake = false;
    };
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixpkgs, nixos-unstable, flake-utils, crate2nix }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
          unstablepkgs = nixos-unstable.legacyPackages.${system};

          crateName = "emojied";

          inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
            generatedCargoNix;

          project = import (generatedCargoNix {
            name = crateName;
            src = ./.;
          }) {
            inherit pkgs;
            defaultCrateOverrides = pkgs.defaultCrateOverrides // {};
          };
      in {
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

        packages.${crateName} = project.rootCrate.build;

        defaultPackage = self.packages.${system}.${crateName};

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
