use crate::{algorithm::{rope_effort::rope_effort::RopeEffort, ropes_count::ropes_count::RopesCount}, kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr, user_setup::user_load_hand::UserLoadHandDevice}};

///
/// Struct to calculate hosting tackle
/// [documentation to calculation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
/// - 'value' - value of hoisting tackle
pub struct HoistingTackle {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl HoistingTackle {
    /// Struct constructor 
    pub fn new() -> Self {
        Self { dbgid: DbgId("HoistingTackle".to_string()),
         value: None 
        }
    }
    ///
    /// Method to calculate hoisting tackle
    /// [documentation to calculation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    pub fn eval(&mut self, mut ropes_count: RopesCount, user_select: &mut Storage,rope_effort: RopeEffort, user_load_device: UserLoadHandDevice) -> Result<f64,StrErr> {
        match self.value {
            Some(hoisting_tackle) => Ok(hoisting_tackle),
            None => {
                match ropes_count.eval(rope_effort, user_select, user_load_device) {
                    Ok(result) => {
                        match result {
                            x if x == 2.0 => {
                                self.value = Some(1.0);
                                Ok(1.0)
                            }
                            _ => {
                                self.value = Some(2.0);
                                Ok(2.0)
                            }
                        }
                    },
                    Err(err) => Err(StrErr(format!("{}.eval | Error: {:?}",self.dbgid,err))),
                }
            }
        }
    }
}