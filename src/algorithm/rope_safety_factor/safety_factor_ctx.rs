///
/// Calculation context store: [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
#[derive(Debug, Clone, Default)]
pub struct SafetyFactorCtx {
    /// value of [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub result: f64,
}
