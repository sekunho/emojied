# emojied

![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/hsekun/emojied)

<p align="center">
  <img src="emojied.png" />
</p>

Shorten your URLs with emojis!

## Features

- Well, shorten your URLs!
- Customize what emoji to use. e.g Want to use an eggplant emoji? Sure, as long
  as it's not taken!
- Not sure what emoji to use? `emojied` autogenerates one for you.
- View URL clicks (simple stats for now)
- Leaderboard - See the top 20 most clicked links!
- Do you hate enabling JS? Well, this works completely fine with JS disabled!\*

## Getting Started

### Processing static assets

Since this is a website, you'll need to deal with the static assets. Fortunately,
it's not that complicated. You just need two things: `esbuild`, and
`tailwindcss`'s CLI. Both of which are available in `nixpkgs`, but if you're a
download-my-own-binary kind of person, then you can use the ff URLs:

  * https://github.com/tailwindlabs/tailwindcss/releases/download/v3.0.23/tailwindcss-linux-x64
  * https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-0.14.27.tgz

Then simply just run the ff:

```sh
tailwindcss \
  --input assets/app.css \
  --output public/app.css \
  --config assets/tailwind.config.js \
  --minify

esbuild assets/app.ts \
  --outfile=public/app.js \
  --minify
```

This will create a stylesheet, and JS file in `public/` respectively.

### Building a static binary

There were some problems when trying to build one through `nix` so it'll be
built with Docker for now using `ekidd/rust-musl-builder`.

#### Fish

```fish
docker run \
  --rm -it \
  -v (pwd):/home/rust/src ekidd/rust-musl-builder \
  cargo build --release
```

#### Bash

```bash
docker run \
  --rm -it \
  -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder \
  cargo build --release
```

This creates a Linux static binary, which can be found in
`./target/x86_64-unknown-linux-musl/release/emojied`. Check if it has any dylibs
with `ldd`:

```sh
> ldd target/x86_64-unknown-linux-musl/release/emojied
        not a dynamic executable
```

`emojied` requires you to provide some environment variables, namely the ff:

- `CA_CERT` (optional): CA certificate's contents. This shouldn't contain the
`BEGIN` and `END` certificate headers. See `bin/run`.
- `PG__HOST` (required)
- `PG__DBNAME` (required)
- `PG__USER` (required)
- `PG__PASSWORD` (required)
- `PG__PORT` (required)
- `PG__POOL_SIZE` (optional, defaults to `22`)
- `PG__CA_CERT` (optional, defaults to No TLS): CA certificate's file path

These aren't required to be present during build-time, only during runtime.

### CA certificate

DBaaS like DO's managed DB requires you to connect to the DB via TLS, and are
required to use the CA certificate they provide.

You have to do two things:

1. Provide the CA certificate's file path with the `PG__CA_CERT` environment
variable; and
2. Dump the CA certificate's contents to that location you just specified.

### Using `Docker`

I wrote a `Dockerfile` that does all that if ever you don't want to. It pretty
much handles all of the above. Wrt the CA cert, you still have to provide the
`CA_CERT`, and `PG__CA_CERT` env variable. Although the latter will have to be
in `/app/ca-certificate.crt`.

While at the project's root, you can build the image with `docker build -t .`.

## Concepts I have to study in more depth

- Traits: Besides the similarities this shares with Haskell's typeclasses, there
  are a lot of things about it that I don't know.
- async
- futures
- tokio
- streams

\* If you're using an extension to disable JS, then this will depend on if they
copied the noscript tag attributes cause the ones I've used don't. It's a bug.
