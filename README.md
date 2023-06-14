# Bazel/Rust/Crates

## Official build using Bazel

```bash
bazel build //...
```

## Rapid local development using Cargo

You'll need a local copy of [protoc][protoc-install] for this:

```bash
cargo build
```

## Repin Crates

```bash
CARGO_BAZEL_REPIN=true bazel build //...
```

[protoc-install]: https://grpc.io/docs/protoc-installation/
