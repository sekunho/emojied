{
  description = "A URL shortener, except emojis";

  inputs = {
    nixpkgs.url = "github:sekunho/nixpkgs?ref=feat/sqitch-sqlite";
    git-hooks.url = "github:cachix/git-hooks.nix";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = { self, nixpkgs, git-hooks, crane, fenix } @ inputs: (
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = (crane.mkLib pkgs).overrideToolchain
        fenix.packages.${system}.stable.toolchain;

      src = pkgs.lib.cleanSourceWith {
        src = ./.;

        filter = path: type:
          (craneLib.filterCargoSources path type)
        ;
      };

      commonArgs = {
        inherit src;
        version = "0.1.5";
        strictDeps = true;
        pname = "emojied";
        name = "emojied";

        buildInputs = [
          pkgs.openssl
        ];

        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      emojied = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
        doCheck = false;
        CARGO_PROFILE = "release";
      });

      emojied-image = pkgs.dockerTools.streamLayeredImage {
        name = "emojied";
        tag = "latest";
        contents = [ self.packages.${system}.emojied ];

        config = {
          Cmd = [ "/bin/emojied" ];
        };
      };
    in
    {
      checks = {
        pre-commit-check = git-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            cargo-check.enable = true;
            clippy.enable = true;
            rustfmt.enable = true;
          };
        };
      };

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

        emojied-docker = emojied-image;
        default = self.packages.${system}.emojied;
      };

      # nix run
      apps.${system}.emojied = {
        type = "app";
        program = "${self.packages.${system}.emojied}/bin/emojied";
      };

      nixosModules.default = import ./nix/modules/services/emojied.nix;
      devShells.${system}.default = craneLib.devShell (import ./nix/shell.nix { inherit pkgs; });
    }
  );
}
