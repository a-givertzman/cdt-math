use crate::{algorithm::{lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi}, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, lifting_class::LiftClass, loading_combination::LoadingCombination}, str_err::str_err::StrErr}};
///
/// Struct, that make calculate the dynamic coefficient
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct DynamicCoefficient {
    dbgid: DbgId,
    value: Option<f64>,
}
//
//
impl DynamicCoefficient {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { dbgid: DbgId(format!("DynamicCoefficient")), value: None }
    }
    ///
    /// Method to calculate the dynamic coefficient
    /// - 'lift_class' - type of lifting class (enum [LiftClass])
    /// - 'driver_type' - lifting mechanism driver type (enum [DriverType])
    /// - 'load_comb' - loading combination (enum [LoadCombination])
    /// - 'vhmax' - rated lifting speed of the mechanism
    /// - 'vhcs' - slow lifting speed of the mechanism
    /// [reference to dynamic coefficient calculating documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self, lift_class: LiftClass, driver_type: DriverType, load_comb: LoadingCombination, vhmax: f64, vhcs: f64) -> Result<f64, StrErr> {
        match self.value {
            Some(val) => Ok(val),
            None => match SelectBettaPhi::new().eval(lift_class) {
                Ok(lift_class) => {
                    match LiftingSpeed::new().eval(driver_type, load_comb, vhmax, vhcs) {
                        Ok(vh) => {
                            let value = lift_class.phi + lift_class.bet * vh;
                            self.value = Some(value);
                            Ok(value)
                        }
                        Err(err) => Err(err)
                    }
                }
                Err(err) => Err(err),
            }
        }
    }
}