use crate::{algorithm::{lifting_speed::lifting_speed::LiftingSpeed, select_bet_phi::select_bet_phi::SelectBetPhi}, kernel::{dbgid::dbgid::DbgId, str_err::str_err::StrErr}};
///
/// Класс, реализующий расчёт динамического коэффициента
/// - 'select_bet_phi' - экземпляр класса [SelectBetPhi]
/// - 'lifting_speed' - экземпляр класса [LiftingSpeed]
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct DynamicCoefficient {
    pub(crate) dbgid: DbgId,
    pub(crate) select_bet_phi: SelectBetPhi,
    pub(crate) lifting_speed: LiftingSpeed,
    pub(crate) value: Option<f64>,
}
impl DynamicCoefficient {
    ///
    /// Конструктор класса DynamicCoefficient
    /// - 'select_bet_phi' - экземпляр класса [SelectBetPhi]
    /// - 'lifting_speed' - экземпляр класса [LiftingSpeed]
    pub fn new(select_bet_phi: SelectBetPhi, lifting_speed: LiftingSpeed) -> Self {
        Self { dbgid: DbgId(format!("DynamicCoefficient")), select_bet_phi, lifting_speed, value: None }
    }
    ///
    /// Метод расчёта динамического коэффициента
    pub fn eval(&mut self) -> Result<f64, StrErr> {
        match self.value {
            Some(val) => Ok(val),
            None => match self.select_bet_phi.eval() {
                Ok(lift_class) => {
                    match self.lifting_speed.eval() {
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