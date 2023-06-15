# Bazel/Rust/Crates

Hybrid Bazel/Cargo project

Released under [MIT License](LICENSE)

## Prerequisites

* [Install Bazel][bazel-install]
* [Install Buildifier][buildifier-install]
* [Install protoc][protoc-install] (Cargo development only)

## Cookbook

### Official build

```bash
bazel build //...
```

### Official test

```bash
bazel test //...
```

### Repin Crates

```bash
CARGO_BAZEL_REPIN=true bazel build //...
```

### Local build

```bash
cargo build
```

### Local test

```bash
cargo test
```

### Run server

```bash
bazel run my-server
```

### Run client

```bash
bazel run my-client
```

[bazel-install]: https://bazel.build/install/
[buildifier-install]: https://github.com/bazelbuild/buildtools
[protoc-install]: https://grpc.io/docs/protoc-installation/
