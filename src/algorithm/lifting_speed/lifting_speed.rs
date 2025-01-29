use crate::kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, loading_combination::LoadingCombination}, initial_data::initial_data::InitialData, str_err::str_err::StrErr};
///
/// Сlass, that select the steady-state lifting speed of the load
/// [reference to steady-state lifting speed documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - value of steady-state lifting speed
/// - 'initial_data' - [InitialData] instance, where store initial data
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    dbgid: DbgId,
    value: Option<f64>,
    initial_data: InitialData
}
//
//
impl LiftingSpeed {
    ///
    /// Class Constructor
    /// - 'initial_data' - [InitialData] instance, where store initial data
    pub fn new(initial_data: InitialData) -> Self {
        Self {
            dbgid: DbgId("LiftingSpeed".to_string()),
            value: None,
            initial_data: initial_data
        }
    }
    ///
    /// Method returns half of the speed
    /// [reference to calculating documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'vhmax' - nominal lifting speed of the mechanism
    fn vhmax_half(vhmax: f64) -> f64 {
        vhmax * 0.5
    }
    ///
    /// Method of calculating the steady-state lifting speed of the load
    /// [reference to steady-state lifting speed choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self) -> f64 {
        match self.value {
            Some(lifting_speed) => return lifting_speed,
            None => {
                let result = match self.initial_data.load_comb {
                    LoadingCombination::A1 | LoadingCombination::B1 => match self.initial_data.driver_type {
                        DriverType::Hd1 => self.initial_data.vhmax,
                        DriverType::Hd2 | DriverType::Hd3 => self.initial_data.vhcs,
                        DriverType::Hd4 => Self::vhmax_half(self.initial_data.vhmax),
                        DriverType::Hd5 => 0.0,
                    },
                    LoadingCombination::C1 => match self.initial_data.driver_type {
                        DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => self.initial_data.vhmax,
                        DriverType::Hd3 | DriverType::Hd5 => Self::vhmax_half(self.initial_data.vhmax),
                    },
                };
                self.value = Some(result);
                result
            },
        }

    }
}
