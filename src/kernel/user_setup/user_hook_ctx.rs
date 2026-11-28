use crate::algorithm::entities::hook::HookBlock;
///
/// Calculation context store: [user hook block](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone, Default)]
pub struct UserHookCtx {
    /// value of [user hook block](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: HookBlock,
}
