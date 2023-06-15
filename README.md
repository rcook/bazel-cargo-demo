# Bazel/Rust/Crates

Demonstration of a hybrid Bazel/Cargo project: use Cargo for rapid, local development
and Bazel for official builds

Released under [MIT License](LICENSE)

## Prerequisites

* [Install Bazel][bazel-install]
* [Install Buildifier][buildifier-install]
* [Install protoc][protoc-install] (Cargo development only)

## Bazel examples

Build:

```bash
bazel build //...
```

Test:

```bash
bazel test //...
```

Repin:

```bash
CARGO_BAZEL_REPIN=true bazel build //...
```

Run server:

```bash
bazel run my-server
```

Run client:

```bash
bazel run my-client
```

## Cargo examples

Build:

```bash
cargo build
```

Test:

```bash
cargo test
```

Run server:

```bash
cargo run --bin my-server
```

Run client:

```bash
cargo run --vin my-client
```

[bazel-install]: https://bazel.build/install/
[buildifier-install]: https://github.com/bazelbuild/buildtools
[protoc-install]: https://grpc.io/docs/protoc-installation/
