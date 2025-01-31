use crate::{algorithm::context::ctx_result::CtxResult, kernel::str_err::str_err::StrErr};
///
/// Struct to store result of algorithm, that calculate steady state lifting speed
#[derive(Debug, Clone, Default)]
pub struct LiftingSpeedCtx {
    /// value of [steady state lifting speed](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: CtxResult<f64, StrErr>,
}
//
//
