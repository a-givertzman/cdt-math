use crate::{algorithm::dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, lift_class::LiftClass, load_combination::LoadCombination}, str_err::str_err::StrErr}};
///
/// Класс, рассчитывающий силу тяжести, действующая на крюк
/// [reference to force of gravity documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct ForceOfGravity{
    dbgid: DbgId,
    g: f64,
    value: Option<f64>,
}
//
impl ForceOfGravity{
    ///
    /// Конструктор класса ForceOfGravity
    pub fn new() -> Self{
        Self {  dbgid: DbgId(format!("ForceOfGravity")),
                g: 9.81,
                value: None,
        }
    }
    ///
    /// Метод расчёта силы тяжести, действующей на крюк
    /// - 'm_to_lift' - масса на крюке
    /// - 'lift_class' - класс подъема (enum [LiftClass])
    /// - 'driver_type' - тип привода механизма подъема (enum [DriverType])
    /// - 'load_comb' - тип комбинации нагрузок (enum [LoadCombination])
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    /// [reference to force of gravity documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self,m_to_lift: f64,lift_class: LiftClass, driver_type: DriverType, load_comb: LoadCombination, vhmax: f64, vhcs: f64) -> Result<f64, StrErr> {
        match self.value {
            Some(val) => Ok(val),
            None => {
                match DynamicCoefficient::new().eval(lift_class, driver_type, load_comb, vhmax, vhcs){
                    Ok(value) => {
                        let fmg = value * m_to_lift * self.g;
                        self.value = Some(fmg);
                        Ok(fmg)
                    },
                    Err(err) => Err(err),
                }
            },
        }
    }
}