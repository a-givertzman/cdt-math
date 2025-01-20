use crate::kernel::{dbgid::dbgid::DbgId, entities::another_load_device::AnLoadDevice};
///
/// Struct to storage user another loading handing device, if it exist
/// - 'an_device' - another loading handing device, which user select
pub struct UserAnLoadDevice {
    dbgid: DbgId,
    pub an_device: Option<AnLoadDevice>

}
//
//
impl UserAnLoadDevice {
    ///
    /// Struct constructor 
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("UserAnLoadDevice".to_string()),
            an_device: None,
        }
    }
    ///
    /// Method to change device
    pub fn change(&mut self, new_name: String, new_weight: f64) {
        match &self.an_device {
            Some(_) => {
                self.an_device = Some(AnLoadDevice{ name: new_name, weight: new_weight });
            },
            None => self.an_device = Some(AnLoadDevice{ name: new_name, weight: new_weight }),
        }
    }
}