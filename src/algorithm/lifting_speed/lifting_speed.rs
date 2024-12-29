use crate::kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, loading_combination::LoadingCombination}, str_err::str_err::StrErr};
///
/// Сlass, that select the steady-state lifting speed of the load
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - value of lifting speed
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    dbgid: DbgId,
    value: Option<f64>,
}
//
//
impl LiftingSpeed {
    ///
    /// Class Constructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("LiftingSpeed".to_string()),
            value: None
        }
    }
    ///
    /// Method returns half of the speed
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'vhmax' - nominal lifting speed of the mechanism
    fn vhmax_half(vhmax: f64) -> f64 {
        vhmax * 0.5
    }
    ///
    /// Method of calculating the steady-state lifting speed of the load
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'driver_type' - type of lifting mechanism driver type (enum [DriverType])
    /// - 'load_comb' - loading combination (enum [LoadingCombination])
    /// - 'vhmax' - nominal lifting speed of the mechanism
    /// - 'vhcs' - slow lifting speed of the mechanism
    pub fn eval(&mut self,driver_type: DriverType,load_comb: LoadingCombination,vhmax: f64,vhcs: f64) -> Result<f64,StrErr> {
        let result = match load_comb {
            LoadingCombination::A1 | LoadingCombination::B1 => match driver_type {
                DriverType::Hd1 => vhmax,
                DriverType::Hd2 | DriverType::Hd3 => vhcs,
                DriverType::Hd4 => Self::vhmax_half(vhmax),
                DriverType::Hd5 => 0.0,
            },
            LoadingCombination::C1 => match driver_type {
                DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => vhmax,
                DriverType::Hd3 | DriverType::Hd5 => Self::vhmax_half(vhmax),
            },
        };
        let result = self.value.get_or_insert(result);
        Ok(result.to_owned())
    }
}
