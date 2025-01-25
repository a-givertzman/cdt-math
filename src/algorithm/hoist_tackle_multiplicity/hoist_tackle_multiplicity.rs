use crate::{algorithm::{hoisting_tackle::hoisting_tackle::HoistingTackle, rope_effort::rope_effort::RopeEffort, ropes_count::ropes_count::RopesCount}, kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr, user_setup::user_load_hand::UserLoadHandDevice}};

///
/// Struct to calculate hoisting tackle multiplicity
/// [documentation to calculation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
/// - 'value' - value of hoisting tackle multiplicity
pub struct HoistTackleMultiplicity {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl HoistTackleMultiplicity {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("HoistTackleMultiplicity".to_string()),
            value: None,
        }
    }
    ///
    /// Method to calculate
    /// [documentation to calculation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    /// - 'ropes_count' - [RopesCount] instance, where store ropes count based on user characteristics
    /// - 'hoist_tackle' - [HoistingTackle] instance, where store hoisting tackle based on user characteristics
    /// - 'user_select' - [Storage] instance, where store user characteristics
    /// - 'user_load_device' - [UserLoadHandDevice] instance, where store info about user loading handing device
    pub fn eval(&mut self, ropes_count: &mut RopesCount, hoist_tackle: &mut HoistingTackle, user_select: &mut Storage, user_load_device: &mut UserLoadHandDevice) -> Result<f64,StrErr> {
        match self.value {
            Some(hoist_tackle_multi) => Ok(hoist_tackle_multi),
            None => {
                let ropes_count_value = ropes_count.eval(RopeEffort::new(), user_select, user_load_device).expect(&format!("{}.eval | Error to calculate ropes count",self.dbgid));
                let hoist_tackle_value = hoist_tackle.eval(ropes_count, user_select, RopeEffort::new(), user_load_device).expect(&format!("{}.eval | Error to calculate hoisting tackle",self.dbgid));
                let result = ropes_count_value/hoist_tackle_value;
                self.value = Some(result);
                Ok(result)
            }
        }
    }
}