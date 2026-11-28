use crate::algorithm::entities::hook::HookBlock;
///
/// Calculation context store: [filtered hook blocks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone, Default)]
pub struct HookBlockFilterCtx {
    /// vector of [filtered hook blocks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: Vec<HookBlock>,
}
