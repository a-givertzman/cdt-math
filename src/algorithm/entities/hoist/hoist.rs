use serde::{Deserialize, Serialize};
use crate::algorithm::entities::hoist::{hoist_driver_type::HoistDriverType, hoist_duty_group::HoistDutyGroup, hoist_headroom::HoistHeadroomType, hoist_manufacturer_type::HoistManufacturerType, hoist_mobility_type::HoistMobilityType, hoist_perfomance_type::HoistPerfomanceType, hoist_type::HoistType, hoist_wheel_count::HoistWheelCount};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
///
/// Represents a [hoist](docs\catalogsPurchasedEquipment.xlsx) with its main characteristics.
pub struct Hoist {
    /// Manufacturer of hoist
    pub manufacturer: HoistManufacturerType,
    /// Name of the hoist model
    pub name: String,
    /// Type of hoist model
    pub hoist_type: HoistType,
    /// Type of hoist driver model
    pub driver_type: HoistDriverType,
    /// Type of hoist mobility
    pub hoist_mobility: HoistMobilityType,
    /// Type of hoist perfomance
    pub hoist_perfomance: HoistPerfomanceType, 
    /// Type of hoist headroom
    pub hoist_headroom: HoistHeadroomType,
    /// Loading capacity of the hoist
    pub load: f64,
    /// Maximum of hoist lifting height
    pub lifting_height: f64,
    /// Type of hoist duty
    pub duty_group: HoistDutyGroup,
    /// Count of hoist wheel
    pub wheel_count: HoistWheelCount,
    /// Nominal lifting speed of hoist
    pub lifting_speed: f64,
    /// Nominal travelling speed of hoist
    pub travelling_speed: f64,
}