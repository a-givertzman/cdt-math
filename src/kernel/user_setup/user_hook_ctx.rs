use crate::algorithm::entities::hook::Hook;
///
/// Struct to store user hook
#[derive(Debug, Clone, Default)]
pub struct UserHookCtx {
    /// value of [user hook](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: Hook,
}
