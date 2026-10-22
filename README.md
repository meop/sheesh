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

On Linux and macOS the target runs under the shim's name, as through a symlink.

`--arg <value>` (repeatable) stamps arguments the target always gets first, before the shim's own:
`kebab --source-path pwsh.exe --target-path shell.exe --arg -nol` runs `pwsh.exe -nol <args>`.

On Windows, `%NAME%` in the stamped path and arguments is the environment variable NAME when the shim runs (one left
unset stays as written), so one machine-wide shim can run each user's own copy of a tool:
`--source-path '%USERPROFILE%\.ghpm\bin\pwsh.exe'`. While the target runs, the shim lets it have Ctrl+C and
Ctrl+Break, and keeps waiting for it rather than exiting first.

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

## Verifying releases

Every release archive contains `LICENSE`. Verify the signed checksum
manifest with the public release key before checking downloaded artifacts:

```sh
gpg --verify SHA256SUMS.sig SHA256SUMS
sha256sum --ignore-missing --check SHA256SUMS
```

## License

[MIT](LICENSE)
