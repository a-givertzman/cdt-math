///
/// Calculation context store: [minimum required breaking force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
#[derive(Debug, Clone, Default)]
pub struct MinBreakForceCtx {
    /// value of [minimum required breaking force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub result: f64,
}
