# sheesh

Shell Executable Embedded Shim. Create tiny wrapper executables that forward execution to a target path stored in the binary itself.

## Binaries

| Binary | Platforms | Purpose |
|--------|-----------|---------|
| `sheesh` | all | Console shim — reads the embedded path and execs the target |
| `sheesh-gui` | Windows | GUI subsystem shim — same as `sheesh` but suppresses the console window |
| `kebab` | all | Stamper tool — copies a shim template and embeds a target path into it |

## How it works

1. `kebab --source-path <target> --target-path <output>` copies the appropriate `sheesh` template and appends the target path as a small binary trailer.
2. When the output executable is run, it reads the trailer, resolves the target path, and execs it — passing all arguments through.

On Windows, `kebab` inspects the PE subsystem of the target to decide whether to use the console or GUI shim template.

## Building

```sh
cargo build --release
```

Produces `target/release/sheesh`, `target/release/sheesh-gui`, and `target/release/kebab`.

## Cross-compile

Add the target with `rustup target add <target>` then build:

```sh
cargo build --release --target aarch64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```
