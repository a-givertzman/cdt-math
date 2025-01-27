use crate::kernel::{dbgid::dbgid::DbgId, entities::{cable_winding_method::CableWingingMethods, mechanism_work_type::MechanismWorkType}, storage::storage::Storage, str_err::str_err::StrErr};

///
/// Struct to choose rope safety factor
/// [documentation to choice](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
/// - 'value' - value of rope safety factor
pub struct RopeSafetyFactor {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl RopeSafetyFactor {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("RopeSafetyFactor".to_string()),
            value: None,
        }
    }
    ///
    /// Method to choose rope safety factor
    /// [documentation to choice](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub fn select(&mut self, user_select: &mut Storage) -> Result<f64,StrErr> {
        match self.value {
            Some(rope_safety_factor) => Ok(rope_safety_factor),
            None => {
                let cable_winding_method: CableWingingMethods = serde_json::from_value::<CableWingingMethods>(user_select.load("test.user_characteristics.cable_winding_method")?).map_err(|err| StrErr(format!("{}.select | Error {:?}",self.dbgid, err)))?;
                let mechanism_work_type: MechanismWorkType = serde_json::from_value::<MechanismWorkType>(user_select.load("test.user_characteristics.mechanism_work_type")?).map_err(|err| StrErr(format!("{}.select | Error {:?}",self.dbgid, err)))?;
                let mut result = 0.0;
                match cable_winding_method {
                    CableWingingMethods::SinglelayerWingind => {
                        match mechanism_work_type {
                            MechanismWorkType::M1 => result = 3.15,
                            MechanismWorkType::M2 => result = 3.35,
                            MechanismWorkType::M3 => result = 3.55,
                            MechanismWorkType::M4 => result = 4.0,
                            MechanismWorkType::M5 => result = 4.5,
                            MechanismWorkType::M6 => result = 5.6,
                            MechanismWorkType::M7 => result = 7.1,
                            MechanismWorkType::M8 => result = 9.0
                        }
                    }
                    CableWingingMethods::MultilayerWinding => {
                        match mechanism_work_type {
                            MechanismWorkType::M1 => result = 3.55,
                            MechanismWorkType::M2 => result = 3.55,
                            MechanismWorkType::M3 => result = 3.55,
                            MechanismWorkType::M4 => result = 4.0,
                            MechanismWorkType::M5 => result = 4.5,
                            MechanismWorkType::M6 => result = 5.6,
                            MechanismWorkType::M7 => return Err(StrErr(format!("{}.select | Error",self.dbgid))),
                            MechanismWorkType::M8 => return Err(StrErr(format!("{}.select | Error",self.dbgid)))
                        }
                    },
                }
                self.value = Some(result);
                Ok(result)
            }
        }
    }
}