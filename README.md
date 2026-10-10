# doctor-template

[`cargo-generate`](https://github.com/cargo-generate/cargo-generate) template for a new doctor CLI on
[doctor-kit](https://github.com/doctor-labs/doctor-kit). Defaults: face **rich**, theme **clinical**.

```bash
cargo install cargo-generate
cargo generate --git https://github.com/doctor-labs/doctor-template template --name foo-doctor
```

Prompts: description, binary name (default = tool name), what it scans, author.
Non-interactive (CI):

```bash
cargo generate --git https://github.com/doctor-labs/doctor-template template --name foo-doctor --silent \
  -d description="Scans Dockerfiles" -d domain="Dockerfiles" -d bin_name="" \
  -d author="You <you@users.noreply.github.com>"
# or: --template-values-file values.toml   (a [values] table with the same keys)
```

You get `src/scanner.rs` (the only file to fill in), `src/checks/` (copy per check), `doctor.toml`,
`theme.toml`, CI, README, AGENTS.md. `doctor-kit` comes from crates.io (`doctor-kit = "0.1"`).
CI here stamps the template and builds the result.
