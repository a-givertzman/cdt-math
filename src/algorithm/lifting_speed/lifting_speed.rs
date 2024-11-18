use crate::kernel::{dbgid::dbgid::DbgId, entities::{driver_type::{self, DriverType}, load_combination::LoadCombination}, str_err::str_err::StrErr};
///
/// Класс, реализующий выбор установившейся скорости подъёма груза дальнейшего расчёта динамического коэффициента
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    pub dbgid: DbgId,
    value: Option<f64>,
}
//
//
//
impl LiftingSpeed {
    ///
    /// Конструктор класса LiftingSpeed
    pub fn new() -> Self {
        Self {
            dbgid: DbgId(format!("LiftingSpeed")),
            value: None
        }
    }
    ///
    /// Метод возвращает половину от скорости, в соответствии (design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn vhmax_half(vhmax: f64) -> f64 {
        vhmax * 0.5
    }
    ///
    /// Метод расчёта установившейся скорости подъёма груза
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'driver_type' - Тип привода мех. под.	(enum [DriverType])
    /// - 'load_comb' - комбинация нагрузок (enum [LoadCombination])
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    pub fn eval(&mut self,driver_type: DriverType,load_comb: LoadCombination,vhmax: f64,vhcs: f64) -> Result<f64, StrErr> {
        let result = match load_comb {
            LoadCombination::A1 | LoadCombination::B1 => match driver_type {
                DriverType::Hd1 => vhmax,
                DriverType::Hd2 | DriverType::Hd3 => vhcs,
                DriverType::Hd4 => Self::vhmax_half(vhmax),
                DriverType::Hd5 => 0.0,
            },
            LoadCombination::C1 => match driver_type {
                DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => vhmax,
                DriverType::Hd3 | DriverType::Hd5 => Self::vhmax_half(vhmax),
            },
        };
        let result = self.value.get_or_insert(result);
        Ok(result.to_owned())
    }
}
