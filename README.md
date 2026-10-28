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
2. When the output executable is run, it reads the trailer, resolves the target path, and runs it — passing all arguments through.

The target runs under the shim's name: whatever reads its command line (`ps`, Task Manager, a terminal multiplexer
saving its session) sees the shim, as it would through a symlink, rather than the versioned path behind it, which a
later upgrade may remove. The target's own path to itself (`/proc/self/exe`, `GetModuleFileName`) stays the real one,
so a tool that finds its files from where it sits still finds them.

`--arg <value>` (repeatable) stamps arguments the target always gets first, before the shim's own:
`kebab --source-path pwsh.exe --target-path shell.exe --arg -nol` runs `pwsh.exe -nol <args>`.

A relative `--source-path` is made absolute from where `kebab` runs. `kebab` stamps the shim beside `--target-path` and
moves it into place, so it can replace a shim that is running, and nothing ever runs one half written.

On Windows, `%NAME%` in the stamped path and arguments is the environment variable NAME when the shim runs (one left
unset stays as written), so one machine-wide shim can run each user's own copy of a tool:
`--source-path '%USERPROFILE%\.ghpm\bin\pwsh.exe'`.

On Windows, `kebab` inspects the PE subsystem of the target to decide whether to use the console or GUI shim template.
A GUI shim that cannot run its target says why in a message box. A batch file is not a target.

### Per platform

The outcome is the same everywhere; how the shim gets there is each platform's own.

|                                  | Linux and macOS                   | Windows                                                     |
| -------------------------------- | --------------------------------- | ----------------------------------------------------------- |
| the shim process                 | becomes the target (`exec`)       | starts the target and waits for it                          |
| the target's command line        | the shim's name, then arguments   | the shim's first word as it was started, then arguments     |
| the shim's own arguments         | passed as the shim got them       | passed as the shim's raw command line had them, unparsed    |
| Ctrl+C and signals               | the target's, directly            | console events left to the target; the shim waits it out    |
| exit code                        | the target's                      | the target's, passed on by the shim                         |
| the shim is killed               | it is the target                  | the target ends with it (a job object)                      |
| what the target starts           | outlives it, as it would          | outlives it: it breaks away from the job                    |
| `%NAME%` in the stamped path     | taken as written                  | expanded when the shim runs                                 |
| replacing a running shim         | a rename over it                  | it is moved aside, and deleted by the next stamp            |

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
