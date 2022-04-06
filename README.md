# emojied

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
- Do you hate enabling JS? Well, this works completely fine with JS disabled!\*

## Getting Started

### Setup

WIP

### Environment variables

- `APP__HOST` (TODO)
- `PG__HOST` (required)
- `PG__DBNAME` (required)
- `PG__USER` (required)
- `PG__PASSWORD` (required)
- `PG__PORT` (required)
- `PG__POOL_SIZE` (optional, defaults to `22`)
- `PG__CA_CERT` (optional, defaults to No TLS)


## Concepts I have to study in more depth

- Traits: Besides the similarities this shares with Haskell's typeclasses, there
  are a lot of things about it that I don't know.
- async
- futures
- tokio
- streams

\* If you're using an extension to disable JS, then this will depend on if they
copied the noscript tag attributes cause the ones I've used don't. It's a bug.
