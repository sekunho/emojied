{
  # Min nix version: 2.7.0
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixos-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixpkgs, nixos-unstable, naersk }: (
    let platform = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${platform};
        unstablepkgs = nixos-unstable.legacyPackages.${platform};

        naersk-lib = naersk.lib.${platform}.override {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };

        shell = import ./nix/shell.nix {
          inherit pkgs;
          inherit unstablepkgs;
        };

        emojied = import ./nix/emojied.nix {
          inherit pkgs;
          inherit unstablepkgs;
          inherit naersk-lib;
        };

        buildDockerImage = tag: pkgs.dockerTools.buildImage {
          name = "emojied-docker";
          tag = tag;
          contents = [ pkgs.bash ];

          config = {
            Cmd = [ "${self.packages.x86_64-linux.emojied}/bin/run" ];
            WorkingDir = "/app";
            Env = [ "PATH=${pkgs.coreutils}/bin/:${self.packages.${platform}.emojied}/bin" ];

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

      packages.${platform} = {
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
        emojied-docker-0-1-1-dev = buildDockerImage "0.1.1-dev";

        default = self.packages.${platform}.emojied;
      };

      # nix run
      apps.${platform}.emojied = pkgs.mkApp {
        drv = self.packages.emojied;

        default = self.packages.${platform}.emojied;
      };

      # BUG: If I use the new default syntax here, `nix-direnv` will complain.
      # It passes `nix flake check` though. But for now, I'll leave it like this.
      #
      # error: flake 'git+file:///home/sekun/Projects/emojiurl' does not provide
      # attribute 'devShells.x86_64-linux.devShell.x86_64-linux',
      # 'packages.x86_64-linux.devShell.x86_64-linux',
      # 'legacyPackages.x86_64-linux.devShell.x86_64-linux',
      # 'devShell.x86_64-linux' or 'defaultPackage.x86_64-linux'
      /* devShell.${platform} = shell; */
      devShell.${platform} = shell;
    }
  );
}
