use serde::{Serialize, Deserialize};

use crate::algorithm::entities::bearing::Bearing;
///
/// Struct to describe type of user request, that ascs user for choosing bearing from filtered
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingQuery {
    /// bearings filtered by user characteristics
    pub filtered_bearings: Vec<Bearing>,
}
//
//
impl ChooseUserBearingQuery {
    pub fn new(filtered_bearings: Vec<Bearing>) -> Self {
        Self {
            filtered_bearings,
        }
    }
}
///
/// Reply to [ChooseUserBearingQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingReply {
    pub choosen_bearing: Bearing
}
//
//
impl ChooseUserBearingReply {
    pub fn new(choosen_bearing: Bearing) -> Self {
        Self {
            choosen_bearing,
        }
    }
}
