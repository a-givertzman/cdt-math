use crate::kernel::dbgid::dbgid::DbgId;
///
/// Struct to storage β2 and ϕ2 coefficients
/// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct BetPhi {
    dbgid: DbgId,
    pub bet: f64,
    pub phi: f64,
}
//
//
impl BetPhi {
    ///
    /// Struct constuctor
    pub fn new(bet: f64, phi: f64,) -> Self {
        Self {
            dbgid: DbgId(format!("SelectBetPhi")),
            bet, 
            phi,
        }
    }
}