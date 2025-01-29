use crate::{algorithm::{lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi}, kernel::{dbgid::dbgid::DbgId, entities::{bet_phi::BetPhi, driver_type::DriverType, lifting_class::LiftClass, loading_combination::LoadingCombination}, str_err::str_err::StrErr}};
///
/// Struct, that make calculate the dynamic coefficient
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - value of dynamic coefficient
#[derive(Debug, Clone)]
pub struct DynamicCoefficient {
    dbgid: DbgId,
    value: Option<f64>,
    select_betta_phi: SelectBettaPhi,
    lifting_speed: LiftingSpeed,
}
//
//
impl DynamicCoefficient {
    ///
    /// Struct constructor
    /// - 'select_betta_phi' - [SelectBettaPhi] instance, where calculating betta and phi coefficients
    /// - 'lifting_speed' - [LiftingSpeed] instance, where calculating steady state lifting speed
    pub fn new(select_betta_phi: SelectBettaPhi, lifting_speed: LiftingSpeed) -> Self {
        Self { 
            dbgid: DbgId("DynamicCoefficient".to_string()),
            value: None,
            select_betta_phi,
            lifting_speed
        }
    }
    ///
    /// Method to calculate the dynamic coefficient
    /// [reference to dynamic coefficient calculating documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self) -> f64 {
        match self.value {
            Some(dynamic_coefficient) => return dynamic_coefficient,
            None => {
                let bet_phi: BetPhi = self.select_betta_phi.eval();
                let lifting_speed: f64 = self.lifting_speed.eval();
                let result = bet_phi.phi + bet_phi.bet * lifting_speed;
                self.value = Some(result);
                return result
            },
        }
    }
}