# AGENTS.md

## After every code change

Run all four — do not skip any:

```sh
cargo build --release
cargo test
cargo fmt
cargo clippy -- -D warnings
```

## Keeping this file current

Update this file whenever a binary or source file is added, removed or renamed,
the trailer format changes, or a convention changes — do not wait to be asked.
If you notice a discrepancy between this file and the code, fix the file
immediately.

Update `README.md` when user-visible behavior changes: a binary's arguments,
what a release ships, or how a shim resolves its target.

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

## Publishing

To cut a release: bump `version` in `Cargo.toml` and push to `main`.

The CI pipeline (`.github/workflows/pipeline.yaml`) runs: Version check (skips
the rest if `v<version>` already exists) → Validate (fmt + clippy + test) →
Build for every target → Publish, which signs `SHA256SUMS`, creates a
GPG-signed tag `v<version>` and publishes the archives to a public GitHub
release. Requires the `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` repository
secrets. Never create the tag by hand.

ghpm and gotm pin an exact sheesh release, by the sha256 of its archives and
of the binaries inside them (in golie). A new sheesh release reaches them only
when that pin is bumped.

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
