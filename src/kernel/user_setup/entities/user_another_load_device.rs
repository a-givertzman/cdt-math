use crate::kernel::{dbgid::dbgid::DbgId, entities::another_load_device::AnLoadDevice, storage::storage::Storage};
///
/// Struct to storage user another loading handing device, if it exist
/// - 'an_device' - another loading handing device, which user select
#[derive(Clone,Debug)]
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
    /// Method to selece another loadign handing device, if it exist
    /// - 'user_select' - [Storage] instance, where user characteristics are store
    pub fn select(&mut self, user_select: &mut Storage) -> Option<AnLoadDevice> {
        match serde_json::from_value::<String>(user_select.load("test.constructions.another_loading_device.name").expect("Error")) {
            Ok(another_load_device_name) => {
                match serde_json::from_value::<f64>(user_select.load("test.constructions.another_loading_device.weight").expect("Error")) {
                    Ok(another_load_device_weight) => {
                        let result = AnLoadDevice { name: another_load_device_name , weight: another_load_device_weight };
                        self.an_device = Some(result.clone());
                        Some(result)
                    },
                    Err(_) => panic!("{}.self | Another loading device should have weight",self.dbgid),
                }
            },
            Err(_) => None,
        }
        
    }
}