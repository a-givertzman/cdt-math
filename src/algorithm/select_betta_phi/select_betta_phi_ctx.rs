use crate::{
    algorithm::{context::ctx_result::CtxResult, entities::bet_phi::BetPhi},
    kernel::str_err::str_err::StrErr,
};
///
/// Struct to store result of algorithm, that calculate β2 and ϕ2 coefficients
#[derive(Debug, Clone, Default)]
pub struct SelectBetPhiCtx {
    /// value of [β2 and ϕ2 coefficients](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: CtxResult<BetPhi, StrErr>,
}
//
//
