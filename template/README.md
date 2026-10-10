# {{project-name}}

{{description}}

Scans {{domain}} and prints a health score. Built on [doctor-kit](https://github.com/doctor-labs/doctor-kit);
generated from [doctor-template](https://github.com/doctor-labs/doctor-template).

```bash
cargo run -- scan <dir>              # rich face, clinical theme (the defaults)
cargo run -- scan <dir> --face compact --theme mono
cargo run -- scan <dir> --json       # doctor/1 envelope; exit 1 when a finding >= --fail-on
cargo run -- shell <dir>             # feature `repl`
cargo run -- explore <dir>           # feature `tui`
```

## How this was generated

```bash
cargo generate --git https://github.com/doctor-labs/doctor-template template --name {{project-name}}
```

## Add a check

1. Copy `src/checks/empty_file.rs` to `src/checks/<name>.rs`; change `ID` (`<category>.<name>`) and `run`.
2. Add `pub mod <name>;` to `src/checks/mod.rs`.
3. Register it in `checks()` in `src/scanner.rs`, and add its category to `categories()` if new.
4. Tune score weights and enable/disable checks in `doctor.toml`; colors in `theme.toml`.

`src/scanner.rs` is the only file you must edit; `src/main.rs` stays thin.

## Note

`doctor-kit` comes from crates.io (`doctor-kit = "0.1"` in `Cargo.toml`).
