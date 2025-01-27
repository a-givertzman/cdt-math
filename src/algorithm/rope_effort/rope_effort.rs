use crate::kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr};
///
/// Struct to make choice of rope effort, based on user loading capacity
/// [documentation to choice](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
/// - 'value' - value of rope effort
pub struct RopeEffort {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl RopeEffort {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            dbgid: DbgId("RopeEffort".to_string()),
            value: None 
        }
    }
    ///
    /// Method to make choice rope effort
    /// [documentation to choice](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    /// - 'user_select' - [Storage] instance, where user characteristics are store
    pub fn eval(&mut self, user_select: &mut Storage) -> Result<f64,StrErr> {
        match self.value {
            Some(rope_effort) => Ok(rope_effort),
            None => {
                let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
                let result = match user_load_capacity {
                    x if x <= 1.0 => 7.5,
                    x if x <= 2.0 => 10.0,
                    x if x <= 6.0 => 20.0,
                    x if x <= 10.0 => 30.0,
                    x if x <= 15.0 => 40.0,
                    x if x <= 20.0 => 50.0,
                    x if x <= 40.0 => 60.0,
                    x if x <= 100.0 => 90.0,
                    x if x <= 150.0 => 130.0,
                    x if x <= 200.0 => 180.0,
                    x if x <= 500.0 => 220.0,
                    _ => {
                        return Err(StrErr(format!("{}.self | Error with storage",self.dbgid)));
                    }
                };
                Ok(result)
            }
        }
    }
}