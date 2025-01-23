use crate::{algorithm::rope_effort::rope_effort::RopeEffort, kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr, user_setup::user_load_hand::UserLoadHandDevice}};

///
/// Struct to calculate ropes count
/// [documentation to calculate](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct RopesCount {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl RopesCount {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            dbgid: DbgId("RopesCount".to_string()), 
            value: None 
        }
    }
    ///
    /// Method to calculate ropes count
    /// [documentation to calculate](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    /// - 'rope_effort' - [RopeEffort] instance, where store rope effort, based on user loading capacity
    /// - 'user_select' - [Storage] instance, where user characteristics are store
    /// - 'user_load_device' - [UserLoadHandDevice] instance, where store info about user loading handing device (hook and maybe another device)
    pub fn eval(&mut self, mut rope_effort: RopeEffort, user_select: &mut Storage, mut user_load_device: UserLoadHandDevice) -> Result<f64,StrErr> {
        match self.value {
            Some(ropes_count) => Ok(ropes_count),
            None => {
                let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
                let rope_efort = rope_effort.eval(user_select).expect(&format!("{}.eval | Error to calculate rope effort", self.dbgid));
                let (summary_weight,payload_weight) = user_load_device.weights(user_select).expect(&format!("{}.eval | Error to calculate rope effort", self.dbgid));
                let result = (user_load_capacity + summary_weight) / rope_efort;
                self.value = Some(result);
                Ok(result)
            }
        }
    }
}