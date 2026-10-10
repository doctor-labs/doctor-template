# {{project-name}}

Doctor CLI on doctor-kit. Edit `src/scanner.rs` (registry) and `src/checks/*` (one file per check).
`src/main.rs` stays thin. Defaults: face rich, theme clinical. Output is the `doctor/1` envelope.
Before pushing: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features`.
TODO: describe what this tool scans ({{domain}}) and any conventions.
