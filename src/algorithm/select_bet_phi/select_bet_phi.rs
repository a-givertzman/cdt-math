use crate::kernel::{dbgid::dbgid::DbgId, entities::{bet_phi::BetPhi, lifting_class::LiftClass}, str_err::str_err::StrErr};
///
/// Struct, that make choice β2 and ϕ2 coefficients, based on user lifting class
/// - 'value' - [BetPhi] instance
/// [reference to betta and phi coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct SelectBetPhi {
    dbgid: DbgId,
    value: Option<BetPhi>,
}
//
//
impl SelectBetPhi {
    ///
    /// Struct conctructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId(format!("SelectBetPhi")),
            value: None,
        }
    }
    ///
    /// Method make choice β2 and ϕ2 coefficients, based on user lifting class
    /// - 'lift_class' - user lifting class (enum [LiftClass]) 
    /// [reference to betta and phi table-choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
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