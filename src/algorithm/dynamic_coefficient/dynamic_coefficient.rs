use crate::{algorithm::{lifting_speed::lifting_speed::LiftingSpeed, select_bet_phi::select_bet_phi::SelectBetPhi}, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::{self, DriverType}, liftclass::{self, LiftClass}, load_combination::LoadCombination}, str_err::str_err::StrErr}};
///
/// Класс, реализующий расчёт динамического коэффициента
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct DynamicCoefficient {
    dbgid: DbgId,
    value: Option<f64>,
}
impl DynamicCoefficient {
    ///
    /// Конструктор класса DynamicCoefficient
    pub fn new() -> Self {
        Self { dbgid: DbgId(format!("DynamicCoefficient")), value: None }
    }
    ///
    /// Метод расчёта динамического коэффициента
    /// - 'lift_class' - класс подъема (enum [LiftClass])
    /// - 'driver_type' - тип привода механизма подъема (enum [DriverType])
    /// - 'load_comb' - тип комбинации нагрузок (enum [LoadCombination])
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    /// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self,lift_class: LiftClass, driver_type: DriverType, load_comb: LoadCombination, vhmax: f64, vhcs: f64) -> Result<f64, StrErr> {
        match self.value {
            Some(val) => Ok(val),
            None => match SelectBetPhi::new().eval(lift_class) {
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