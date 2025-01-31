///
/// Struct to storage β2 and ϕ2 coefficients
/// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'bet' - β2 coefficient
/// - 'phi' - ϕ2 coefficient
#[derive(Debug, Clone, PartialEq)]
pub struct BetPhi {
    pub bet: f64,
    pub phi: f64,
}
//
//
impl BetPhi {
    ///
    /// Struct constuctor
    /// - 'bet' - β2 coefficient
    /// - 'phi' - ϕ2 coefficient
    pub fn new(bet: f64, phi: f64,) -> Self {
        Self {
            bet, 
            phi,
        }
    }
}