use crate::{algorithm::context::ctx_result::CtxResult, kernel::str_err::str_err::StrErr};
///
/// Struct to store result of algorithm, that calculate dynamic coefficient
#[derive(Debug, Clone, Default)]
pub struct DynamicCoefficientCtx {
    /// value of [dynamic coefficient](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: CtxResult<f64, StrErr>,
}
//
//