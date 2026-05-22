# CLAUDE.md

Build/test/lint commands for this project.

## Build

```sh
cargo build --release
```

## Test

```sh
cargo test
```

## Lint

```sh
cargo clippy -- -D warnings
cargo fmt --check
```

## Cross-compile

All targets build natively — no cross-compilation tooling required.

```sh
cargo build --release --target aarch64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin   # cross-compiled from macOS arm64
cargo build --release --target aarch64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

Add a target with `rustup target add <target>` before building for it.

## Project structure

- `src/lib.rs` — shim runtime: reads the embedded trailer and execs the target
- `src/data.rs` — binary trailer format (append/read the embedded path at end of exe)
- `src/windows_pe.rs` — Windows PE subsystem detection (GUI vs Console)
- `src/platform/` — platform dispatch for `spawn_target` (unix: exec, windows: spawn+wait)
- `src/bin/sheesh.rs` — console shim entry point
- `src/bin/sheesh-gui.rs` — GUI subsystem shim entry point
- `src/bin/kebab.rs` — stamper tool: copies shim template and embeds target path

## Conventions

- Three binaries in one crate: `sheesh`, `sheesh-gui`, `kebab`
- All three compile on every platform; only Windows releases package `sheesh-gui`
  (the `#![cfg_attr(windows, windows_subsystem = "windows")]` attribute is a no-op on Unix)
- `kebab` selects the GUI or console shim template based on PE subsystem of the source binary
- Trailer format: `[path bytes][u32 path_len LE][b"SHIM"]` appended to the exe
- Version is in `Cargo.toml`; CI creates a signed tag on version bump
