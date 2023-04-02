{
  description = "A URL shortener, except emojis";

  inputs = {
    nixospkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    naersk.url = "github:nix-community/naersk";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixospkgs, devenv, naersk, pre-commit-hooks } @ inputs: (
    let system = "x86_64-linux";
        pkgs = nixospkgs.legacyPackages.${system};

        naersk-lib = naersk.lib.${system}.override {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };

        shell = import ./nix/shell.nix {
          inherit pkgs;
        };

        emojied = import ./nix/modules/packages/emojied.nix {
          inherit pkgs;
          inherit naersk-lib;
        };

        buildDockerImage = tag: pkgs.dockerTools.buildImage {
          name = "emojied-docker";
          tag = tag;
          contents = [ pkgs.bash ];

          config = {
            Cmd = [ "${self.packages.x86_64-linux.emojied}/bin/run" ];
            WorkingDir = "/app";
            Env = [ "PATH=${pkgs.coreutils}/bin/:${self.packages.${system}.emojied}/bin" ];

            ExposedPorts = {
              "3000/tcp" = {};
            };
          };
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

      packages.${system} = {
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
        emojied-docker = buildDockerImage "latest";

        default = self.packages.${system}.emojied;
      };

      # nix run
      apps.${system}.emojied = {
        type = "app";
        program = "${self.packages.${system}.emojied}/bin/emojied";
      };

      nixosModule = import ./nix/modules/services/emojied.nix;

      devShells.${system}.default = devenv.lib.mkShell {
        inherit inputs pkgs;

        modules = [
          ./nix/shell.nix
        ];
      };
    }
  );
}
