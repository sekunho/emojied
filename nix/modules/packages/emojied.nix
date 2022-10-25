{ pkgs, unstablepkgs, naersk-lib }:

(naersk-lib.buildPackage {
  pname = "emojied";
  version = "0.1.3";
  root = ../../../.;
  nativeBuildInputs = with pkgs; [ ];
  buildInputs = with pkgs; [ openssl pkg-config ];
}).overrideAttrs (old: {
  nativeBuildInputs = old.nativeBuildInputs ++ [
    unstablepkgs.nodePackages.typescript
    unstablepkgs.nodePackages.tailwindcss
    unstablepkgs.esbuild
  ];

  doCheck = true;

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
})
