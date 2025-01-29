use crate::kernel::{dbgid::dbgid::DbgId, entities::{bet_phi::BetPhi, lifting_class::LiftClass}, initial_data::initial_data::InitialData, str_err::str_err::StrErr};
///
/// Struct, that make choice β2 and ϕ2 coefficients, based on user lifting class
/// [reference to betta and phi coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - [BetPhi] instance
/// - 'initial_data' - [InitialData] instance, where store initial data
#[derive(Debug, Clone)]
pub struct SelectBettaPhi {
    dbgid: DbgId,
    value: Option<BetPhi>,
    initial_data: InitialData
}
//
//
impl SelectBettaPhi {
    ///
    /// Class Constructor
    /// - 'initial_data' - [InitialData] instance, where store initial data
    pub fn new(initial_data: InitialData) -> Self {
        Self {
            dbgid: DbgId("SelectBetPhi".to_string()),
            value: None,
            initial_data: initial_data
        }
    }
    ///
    /// Method make choice β2 and ϕ2 coefficients, based on user lifting class
    /// [reference to betta and phi table-choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'lift_class' - user lifting class (enum [LiftClass]) 
    pub fn eval(&mut self) -> BetPhi {
        match &self.value {
            Some(bet_phi) => return bet_phi.clone(),
            None => {
                let result = match self.initial_data.lift_class {
                    LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
                    LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
                    LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
                    LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
                };
                self.value = Some(result.clone());
                return result
            },
        }
    }
}