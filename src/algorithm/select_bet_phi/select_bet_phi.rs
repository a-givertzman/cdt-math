use crate::kernel::{dbgid::dbgid::DbgId, entities::liftclass::LiftClass, str_err::str_err::StrErr};
///
/// Класс реализующий выбор коэффициентов Betta и Phi, относительно класса подъема
/// - 'value' - экземпляр класса [BetPhi]
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct SelectBetPhi {
    dbgid: DbgId,
    value: Option<BetPhi>,
}
impl SelectBetPhi {
    ///
    /// Конструктор класса SelectBetPhi 
    pub fn new() -> Self {
        Self {
            dbgid: DbgId(format!("SelectBetPhi")),
            value: None,
        }
    }
    ///
    /// Метод выбора коэффициентов Betta и Phi, относительно класса подъема
    /// - 'lift_class' - класс подъема (enum [LiftClass]) 
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self,lift_class: LiftClass) -> Result<BetPhi, StrErr> {
        let result = match lift_class {
            LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
            LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
            LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
            LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
            _ => return Err(format!("{}.eval | Invalid HookLiftClassId: {:?}", self.dbgid, lift_class).into()),
        };
        let result = self.value.get_or_insert(result);
        Ok(result.to_owned())
    }
}
///
/// Класс для хранение значений bet и phi
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct BetPhi {
    pub bet: f64,
    pub phi: f64,
}
impl BetPhi {
    ///
    /// Конструктор класса BetPhi
    pub fn new(bet: f64, phi: f64,) -> Self {
        Self { bet, phi }
    }
}