use crate::algorithm::entities::hoisting_rope::hoisting_rope::HoistingRope;
///
/// Calculation context store: [user hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
#[derive(Debug, Clone, Default)]
pub struct UserHoistRopeCtx {
    /// value of [user hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub result: HoistingRope,
}
