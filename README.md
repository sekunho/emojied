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

\* If you're using an extension to disable JS, then this will depend on if they
copied the noscript tag attributes cause the ones I've used don't. It's a bug.

## Getting Started

For the dev environment, it's fairly simple to set up the project if you're
already using Nix (with Flakes). If not, then you'll need to get the dependencies
yourself; sorry!

Here are the versions of the important ones:

- `rustc`: 1.58.1
- `cargo`: 1.58.0
- `postgresql`: 14.1
- `sqitch`: 1.1.0
- `tailwindcss`: 3.0.23
- `esbuild`: 0.14.27
- `typescript`: 4.6.2
- `openssl`: 1.1.1n 15 Mar 2022

Everything else doesn't matter too much I think. You could probably just use
whatever version of `rust-analyzer` you have, for example.

However, if you _do_ have Nix 2.7.0 (with Flakes), you probably know how to
use it anyway. `nix develop` (or don't if you have `nix-direnv` already), and
dev away.

Oh, you also have to set up the database called `emojied_db` (locally).

Once you've done whatever to get all the dependencies, you can do the ff:

```
sqitch deploy

# You can also do `cargo run` if you don't want to use `cargo-watch`.
PG__DBNAME="YOUR_DB_NAME_HERE" PG__HOST="localhost" PG__USER="YOUR_USER_HERE" PG__PORT="5432" A
PP__STATIC_ASSETS="./public" cargo watch -x run
```

This should run a server in port `3000`, which you can access in http://localhost:3000.

Oh, you want to set it up for a prod server? But why? (continue reading)

### Prepping the binary for prod

You'll need the binary to run the server. You can prep it in a few ways:

#### Docker image

A docker image is available here: https://hub.docker.com/r/hsekun/emojied

#### Pre-built binary

NOTE: The binary requires some dynamic libs. Haven't figured
out a way to get a static binary working. I wasn't able to double check properly.

#### Build from source

You can build the binary, and static assets with `nix`. You can also build it
some other way if you prefer, but I'm not gonna bother with that.

Options:

1. `nix build`: Builds `emojied`'s + static assets, and provides
an `APP__STATIC_ASSETS` environment variable. This is the "wrapped" version.
2. `nix build .#emojied-unwrapped`: Like above, but doesn't provide the env
variable.

In both, everything is already taken care of. \#1 is more suitable for distributing
it as an application.

#### Build a `Docker` image

If ever you need a Docker image with `emojied`, then you'll need `nix` (flakes)
to build it.

```sh
# Build the Docker image tar
nix build .#emojied-docker

# Load result to Docker
docker load < result
```

From this, you get a Docker image `emojied-docker:latest`! If you want an example,
you can check out the `.github/workflows/main.yml`.

### Environment variables

`emojied` requires you to provide some environment variables, namely the ff:

- `APP__STATIC_ASSETS` (required, path that directly contains `app.css`, etc.):
Path of `public/`
- `PG__HOST` (required)
- `PG__DBNAME` (required)
- `PG__USER` (required)
- `PG__PASSWORD` (required)
- `PG__PORT` (required)
- `PG__POOL_SIZE` (optional, defaults to `22`)
- `PG__CA_CERT` (optional, defaults to No TLS): CA certificate's file path
- `CA_CERT` (optional): CA certificate's contents. This shouldn't contain the
`BEGIN` and `END` certificate headers. See `bin/run`.

### Schema migrations

Finally, you'll need to migrate the database `emojied` will use. I'm using
`sqitch` cause I'm poor, and need a schema migration tool that doesn't come with
broken kneecaps in the free tier. `sqitch` gets the job done here, and it's OSS.

```sh
SQITCH_PASSWORD="YOUR_DB_PASSWORD_HERE" sqitch deploy \
  --db-host YOUR_DB_HOST_HERE \
  --db-port YOUR_DB_PORT_HERE \
  --db-user YOUR_DB_ADMIN_USERNAME_HERE \
  --db-name YOUR_DB_NAME_HERE
```

Of course, replace the `YOUR_*_HERE` with your actual database credentials. It
should say `ok` for everything. If it doesn't then you probably set something
up in the DB incorrectly. Oh, the device you're running this command on, whether
it be local or in some CI, it needs to be added to the trusted sources so that
you can actually communicate with the DB. I just temporarily add my PC as one
of the trusted sources, then remove it after I've performed the migrations.

