use crate::algorithm::entities::hoisting_rope::hoisting_rope::HoistingRope;
///
/// Calculation context store: [filtered hoisting ropes](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
#[derive(Debug, Clone, Default)]
pub struct HoistRopeFilterCtx {
    /// vector of [filtered hoisting ropes](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub result: Vec<HoistingRope>,
}
