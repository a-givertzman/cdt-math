use crate::{
    algorithm::{context::ctx_result::CtxResult, entities::hook::Hook},
    kernel::str_err::str_err::StrErr,
};
///
/// Struct to store result of algorithm, that filter hooks by user loading capacity
#[derive(Debug, Clone, Default)]
pub struct HookFilterCtx {
    /// vector of [filtered hooks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: CtxResult<Vec<Hook>, StrErr>,
}
//
//
