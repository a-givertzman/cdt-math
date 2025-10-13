use std::str::FromStr;

use serde::{Serialize, Deserialize};
use crate::algorithm::entities::hoist::{hoist_headroom::HoistHeadroomType, hoist_manufacturer_type::HoistManufacturerType, hoist_type::HoistType, hoist_wheel_count::HoistWheelCount};
///
/// User request | Asks user for choose characteristics of [Hoist] to filter from all data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChooseHoistCharactetisticsQuery {
    pub hoist_type: Vec<String>, 
    pub hoist_count_wheel: Vec<String>, 
    pub headroom_type: Vec<String>, 
    pub hoist_manufacturer: Vec<String>,
    #[serde(skip_serializing_if = "testing")]
    pub testing: bool,
}
//
//
impl ChooseHoistCharactetisticsQuery {
    ///
    /// New instance [ChooseHoistCharactetisticsQuery]
    pub fn new(hoist_type: Vec<String>, hoist_count_wheel: Vec<String>, headroom_type: Vec<String>, hoist_manufacturer: Vec<String>) -> Self {
        Self {
            hoist_type,
            hoist_count_wheel,
            headroom_type,
            hoist_manufacturer,
            testing: false,
        }
    }
    ///
    /// New instance [ChooseHoistCharactetisticsQuery] for testing
    pub fn test(hoist_type: Vec<String>, hoist_count_wheel: Vec<String>, headroom_type: Vec<String>, hoist_manufacturer: Vec<String>) -> Self {
        Self {
            hoist_type,
            hoist_count_wheel,
            headroom_type,
            hoist_manufacturer,
            testing: true,
        }
    }
}
///
/// Used for `skip_serializing_if`
fn testing(v: &bool) -> bool {
    !v
}
///
/// Reply to [ChooseHoistCharactetisticsQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseHoistCharactetisticsReply {
    pub hoist_type: Result<HoistType,String>,
    pub hoist_count_wheel: HoistWheelCount, 
    pub headroom_type: HoistHeadroomType, 
    pub hoist_manufacturer: HoistManufacturerType,
}
//
//
impl ChooseHoistCharactetisticsReply {
    ///
    /// New instance [ChooseHoistCharactetisticsReply]
    pub fn new(hoist_type: String, hoist_count_wheel: HoistWheelCount, headroom_type: HoistHeadroomType, hoist_manufacturer: HoistManufacturerType) -> Self {
        let mut hoist_self: Result<HoistType, String> = Result::Ok(HoistType::default());
        if hoist_type == "Any" {
            hoist_self = Err(hoist_type);
        } else {
            hoist_self = Ok(HoistType::from_str(&hoist_type).unwrap());
        };
        Self {
            hoist_count_wheel,
            headroom_type,
            hoist_manufacturer,
            hoist_type: hoist_self,
        }
    }
}
