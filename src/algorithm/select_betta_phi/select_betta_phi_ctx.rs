use crate::
    algorithm::entities::bet_phi::BetPhi
;
///
/// Struct to store result of algorithm, that calculate β2 and ϕ2 coefficients
#[derive(Debug, Clone, Default)]
pub struct SelectBetPhiCtx {
    /// value of [β2 and ϕ2 coefficients](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub result: BetPhi,
}
//
//
