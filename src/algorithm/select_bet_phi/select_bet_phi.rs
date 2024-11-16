use crate::kernel::{dbgid::dbgid::DbgId, entities::liftclass::LiftClass, str_err::str_err::StrErr};


///
/// Select Bet & Phi dipending on...
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct SelectBetPhi {
    dbgid: DbgId,
    lift_class: LiftClass,
    value: Option<BetPhi>,
}
impl SelectBetPhi {
    ///
    /// Returns SelectBetPhi new instance
    /// - `lift_class` - [HookLiftClassId]() 
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn new(lift_class: LiftClass) -> Self {
        Self {
            dbgid: DbgId(format!("SelectBetPhi")),
            lift_class,
            value: None,
        }
    }
    ///
    /// Few words about eval method
    /// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self) -> Result<BetPhi, StrErr> {
        let result = match self.lift_class {
            LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
            LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
            LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
            LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
            _ => return Err(format!("{}.eval | Invalid HookLiftClassId: {:?}", self.dbgid, self.lift_class).into()),
        };
        let result = self.value.get_or_insert(result);
        Ok(result.to_owned())
    }
}
///
/// Just holding Bet & Phi
/// [reference to Lift Class documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct BetPhi {
    pub bet: f64,
    pub phi: f64,
}
impl BetPhi {
    pub fn new(bet: f64, phi: f64,) -> Self {
        Self { bet, phi }
    }
}