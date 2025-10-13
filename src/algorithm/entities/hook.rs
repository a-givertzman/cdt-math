use serde::{Deserialize, Serialize};

use crate::algorithm::entities::hoist_group::HoistGroup;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
///
/// Represents a [hook block](docs\catalogsPurchasedEquipment.xlsx) with specifications and load capacities.
pub struct HookBlock {
    /// GOST number of hook
    pub gost: String,
    /// hook type
    pub r#type: String,
    /// loading capacity for [M1-M3 types of mechanism work](design\docs\algorithm\part01\initial_data.md)
    pub load_m13: f64,
    /// loading capacity for [M4-M6 types of mechanism work](design\docs\algorithm\part01\initial_data.md)
    pub load_m46: f64,
    /// loading capacity for [M7-M8 types of mechanism work](design\docs\algorithm\part01\initial_data.md)
    pub load_m79: f64,
    /// shank diameter
    pub shank_diameter: f64,
    /// weight of hook
    pub weight: f64,
}
//
//
impl HookBlock {
    ///
    /// Return loading capacity for user hoist group
    pub fn hoist_load(self, hoist_group: HoistGroup) -> f64 {
        match hoist_group {
            HoistGroup::M1 | HoistGroup::M2 | HoistGroup::M3 => return self.load_m13,
            HoistGroup::M4 | HoistGroup::M5 | HoistGroup::M6 => return self.load_m46,
            HoistGroup::M7 | HoistGroup::M8 | HoistGroup::M9 => return self.load_m79,
        }
    }
}
