# tofnd -- A threshold signature scheme daemon

A gRPC server wrapper for the [tofn](https://github.com/axelarnetwork/tofn) library.

## Status

Under active development.

## Setup

`tofnd` uses the [hyperium/tonic](https://github.com/hyperium/tonic) Rust gRPC implementation, which requires:
* Rust `1.39` or greater for the `async_await` feature
    ```
    $ rustup update
    ```
* `rustfmt` to tidy up the code it generates
    ```
    $ rustup component add rustfmt
    ```

## Run the server

```
$ cargo run
```
or specify listen port:
```
$ cargo run -- 50051
```
Terminate the server with `ctrl+C`.

The gRPC service is called `GG20`.

## Run the tests

```
$ cargo test
```