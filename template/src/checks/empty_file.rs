// Example check. Copy this per check: new file, new `ID`, register it in scanner.rs.
use doctor_core::Severity;
use doctor_kit::{finding, Check, CheckResult, Ctx};

/// Stable id: doctor.toml `[checks]` enables/disables by this.
pub const ID: &str = "files.empty";
pub const CATEGORY: &str = "files";

pub struct EmptyFile;

impl Check for EmptyFile {
    fn id(&self) -> &'static str {
        ID
    }
    fn category(&self) -> &'static str {
        CATEGORY
    }
    /// Optional gate: when false the check is skipped (listed under `data.skipped`).
    fn applies(&self, ctx: &Ctx) -> bool {
        !ctx.files.is_empty()
    }
    fn run(&self, ctx: &Ctx) -> CheckResult {
        let findings = ctx
            .files
            .iter()
            .filter(|f| std::fs::metadata(ctx.root.join(f)).is_ok_and(|m| m.len() == 0))
            .map(|f| {
                let mut x = finding(ID, CATEGORY, Severity::Low, "File is empty", f, None);
                x.remedy = Some("Delete it or add content.".into());
                x
            })
            .collect();
        CheckResult::findings(findings).advice("Empty files are usually leftovers.")
    }
}
