use serde::{Deserialize, Serialize};
use crate::algorithm::entities::hoist::{hoist_driver_type::HoistDriverType, hoist_type::HoistType};
#[derive(Serialize, Deserialize, Debug, Clone)]
///
/// Represents a [hoist](docs\catalogsPurchasedEquipment.xlsx) with its main characteristics.
pub struct Hoist {
    /// Name of the hoist model
    pub name: String,
    /// Type of hoist model
    pub hoist_type: HoistType,
    /// Type of hoist driver model
    pub driver_type: HoistDriverType,
    /// Outer diameter of the bearing
    pub outer_diameter: f64,
    /// Inner diameter of the bearing
    pub inner_diameter: f64,
    /// Static load capacity of the bearing
    pub static_load: f64,
    /// Height of the bearing
    pub height: f64,
}