use crate::kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, load_combination::LoadCombination}, str_err::str_err::StrErr};
///
/// Класс, реализующий выбор установившейся скорости подъёма груза дальнейшего расчёта динамического коэффициента
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'driver_type' - Тип привода мех. под.	
/// - 'load_comb' - комбинация нагрузок
/// - 'vhmax' - номинальная скорость подъёма механизма
/// - 'vhcs' - замедленная скорость подъёма механизма
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    pub dbgid: DbgId,
    pub(crate) driver_type: DriverType,
    pub(crate) load_comb: LoadCombination,
    pub(crate) vhmax: f64,
    pub(crate) vhcs: f64,    
    pub(crate) value: Option<f64>,
}
//
//
//
impl LiftingSpeed {
    ///
    /// Конструктор класса LiftingSpeed
    /// - 'driver_type' - Тип привода мех. под.	
    /// - 'load_comb' - комбинация нагрузок
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    pub fn new(
        driver_type: DriverType,
        load_comb: LoadCombination,
        vhmax: f64,
        vhcs: f64
    ) -> Self {
        Self {
            dbgid: DbgId(format!("LiftingSpeed")),
            driver_type,
            load_comb,
            vhmax,
            vhcs,
            value: None
        }
    }
    ///
    /// Метод возвращает половину от числа, в соответствии с [...](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn vhmax_half(&self) -> f64 {
        self.vhmax * 0.5
    }
    ///
    /// Метод расчёта установившейся скорости подъёма груза
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self) -> Result<f64, StrErr> {
        let result = match self.load_comb {
            LoadCombination::A1 | LoadCombination::B1 => match self.driver_type {
                DriverType::Hd1 => self.vhmax,
                DriverType::Hd2 | DriverType::Hd3 => self.vhcs,
                DriverType::Hd4 => self.vhmax_half(),
                DriverType::Hd5 => 0.0,
            },
            LoadCombination::C1 => match self.driver_type {
                DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => self.vhmax,
                DriverType::Hd3 | DriverType::Hd5 => self.vhmax_half(),
            },
        };
        let result = self.value.get_or_insert(result);
        Ok(result.to_owned())
    }
}
