use crate::kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr};
use super::entities::{user_another_load_device::UserAnLoadDevice, user_hook::UserHook};
///
/// Struct to storage summary info about user loading handing device
/// - 'hook' - user hook, based on his characteristics
/// - 'an_device' - user another loading handing device
/// - 'summary-weight' - summary weight of all loading handing device
/// - 'payload mass' - payload mass of loading handing device
/// [documentation to summary weight and payload mass](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
pub struct UserLoadHandDevice {
    dbgid: DbgId,
    hook: UserHook,
    an_device: UserAnLoadDevice,
    summary_weight: f64,
    payload_mass: f64
}
//
//
impl UserLoadHandDevice {
    ///
    /// Struct constructor 
    /// - 'hook' [UserHook] instance, where store hook, based on user characteristics
    /// - 'an_device' - [UserAnLoadDevice] instance, where store another loading handing device, which user select
    pub fn new(hook: UserHook, an_device: UserAnLoadDevice) -> Self {
        Self {
            dbgid: DbgId("UserLoadHandDevice".to_string()),
            hook,
            an_device,
            summary_weight: 0.0,
            payload_mass: 0.0
        }
    }
    ///
    /// Method to calculate summary weight and payload mass, that returns tuple (summary weight, payload mass)
    /// [documentation to calculation](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    /// - 'user_select' - [Storage] instance, where user characteristics are store
    pub fn weights(&mut self, user_select: &mut Storage) -> Result<(f64,f64),StrErr> {
        let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        match &self.hook.hook {
            Some(hook) => {
                match &self.an_device.an_device {
                    Some(an_device) => Ok((hook.weight+an_device.weight,user_load_capacity-an_device.weight)),
                    None => Ok((hook.weight,user_load_capacity)),
                }
            },
            None => Err(StrErr(format!("{}.eval | User didn't choose any hook yet",self.dbgid)))
        }
    }
}