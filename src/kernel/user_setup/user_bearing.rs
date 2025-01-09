use crate::kernel::{dbgid::dbgid::DbgId, entities::bearing::Bearing, str_err::str_err::StrErr};
///
/// Struct to choose the bearing from filtered, based on user characteristics
pub struct UserBearing {
    dbgid: DbgId,
    pub bearing: Option<Bearing>
}
//
//
impl UserBearing {
    ///
    /// Struct Constructor
    pub fn new() -> Self {
        Self { 
            dbgid: DbgId("UserBearing".to_string()),
            bearing: None 
        }
    }
    ///
    /// Method to select
    /// - 'filtered_bearings' - all filtered bearings (vector of [Bearing] instance)
    /// - 'user_choice' - just usize value from keyboard to make select
    pub fn select(&mut self, filtered_bearings: Vec<Bearing>, user_choice: usize) -> Result<Bearing,StrErr> {
        match filtered_bearings.get(user_choice) {
            Some(bearing) => {
                self.bearing = Some(bearing.clone());
                Ok(bearing.clone()) 
            },
            None => Err(StrErr(format!("{}.select | There is no bearing with this index: {}", self.dbgid, user_choice))),
        }
    }
}