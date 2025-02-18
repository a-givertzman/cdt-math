use serde::{Serialize, Deserialize};

use crate::algorithm::entities::bearing::Bearing;
///
/// User request | Asks user for choose [Bearing] from filtered
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingQuery {
    /// vector of bearings filtered by user characteristics
    pub variants: Vec<Bearing>,
}
//
//
impl ChooseUserBearingQuery {
    pub fn new(variants: Vec<Bearing>) -> Self {
        Self {
            variants,
        }
    }
}
///
/// Reply to [ChooseUserBearingQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingReply {
    pub answer: Bearing
}
//
//
impl ChooseUserBearingReply {
    pub fn new(choosen: Bearing) -> Self {
        Self {
            answer: choosen,
        }
    }
}
