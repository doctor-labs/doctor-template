//! {{description}}
//!
//! Thin on purpose: the CLI (scan / shell / explore, --face, --theme, --json, exit codes) comes
//! from doctor-kit. Defaults: face = rich, theme = clinical. Put your logic in `scanner.rs`.
mod checks;
mod scanner;

fn main() -> std::process::ExitCode {
    doctor_kit::run(scanner::Scanner)
}
