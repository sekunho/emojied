{
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixos-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };
outputs = { self, nixpkgs, nixos-unstable, utils, naersk }: utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = nixpkgs.legacyPackages.${system};
          unstablepkgs = nixos-unstable.legacyPackages.${system};
          naersk-lib = naersk.lib."${system}".override {
            cargo = pkgs.cargo;
            rustc = pkgs.rustc;
          };

          emojied = (naersk-lib.buildPackage {
            pname = "emojied";
            root = ./.;
            gitSubmodules = true;
            nativeBuildInputs = with pkgs; [ ];
            buildInputs = with pkgs; [ openssl pkg-config ];
          }).overrideAttrs (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [
              unstablepkgs.nodePackages.typescript
              unstablepkgs.nodePackages.tailwindcss
              unstablepkgs.esbuild
            ];

            buildInputs = old.buildInputs;

            buildPhase = old.buildPhase + ''
              tailwindcss \
                --input assets/app.css \
                --config assets/tailwind.config.js \
                --output public/app.css \
                --minify

              esbuild \
                assets/app.ts \
                --outdir=public/ \
                --minify
            '';

            installPhase = old.installPhase + ''
              mv bin/run $out/bin/run
              mv public $out/bin
            '';
          });
      in rec
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

        packages = {
          emojied-unwrapped = emojied;

          emojied = pkgs.symlinkJoin {
            name = "emojied";
            paths = [ emojied ];
            buildInputs = [ pkgs.makeWrapper ];

            # https://gist.github.com/CMCDragonkai/9b65cbb1989913555c203f4fa9c23374
            postBuild = ''
              wrapProgram $out/bin/emojied \
                --set APP__STATIC_ASSETS "${emojied}/bin/public"
            '';
          };

          # https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/docker/examples.nix
          emojied-docker = pkgs.dockerTools.buildImage {
            name = "emojied-docker";
            tag = "latest";
            contents = [ pkgs.coreutils packages.emojied pkgs.bash ];

            config = {
              Cmd = [ "${packages.emojied}/bin/run" ];
              WorkingDir = "/app";
              Env = [ "PATH=${pkgs.coreutils}/bin/:${packages.emojied}/bin" ];

              ExposedPorts = {
                "3000/tcp" = {};
              };
            };
          };
        };

        defaultPackage = packages.emojied;

        # nix run
        apps.emojied = utils.lib.mkApp {
          drv = packages.emojied;
        };


        defaultApp = apps.emojied;

        devShells.ci = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.openssl
            pkgs.pkg-config

            unstablepkgs.nodePackages.typescript
            unstablepkgs.nodePackages.tailwindcss
            unstablepkgs.esbuild
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
            pkgs.zip
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      });
}
