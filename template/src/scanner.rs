//! THE file you fill in: what `{{bin_name}}` scans ({{domain}}) and which checks it runs.
use crate::checks::{self, empty_file::EmptyFile};
use doctor_kit::{Check, Doctor, Theme};

pub struct Scanner;

impl Doctor for Scanner {
    fn name(&self) -> &str {
        "{{bin_name}}"
    }
    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    /// Display order. Findings in a category not listed here are dropped.
    fn categories(&self) -> Vec<&'static str> {
        vec![checks::empty_file::CATEGORY]
    }
    /// TODO: register your checks here (explicit list, no reflection).
    fn checks(&self) -> Vec<Box<dyn Check>> {
        vec![Box::new(EmptyFile)]
    }
    /// Default theme is clinical; `--theme` and theme.toml still override it.
    fn theme(&self) -> Theme {
        Theme::clinical()
    }
}
