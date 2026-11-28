use sal_sync::services::entity::{dbg_id::DbgId, error::str_err::StrErr};
use crate::{
    algorithm::entities::hoisting_rope::{
        hoisting_rope::HoistingRope, 
        rope_durability_class::RopeDurabilityClass, 
        rope_type::RopeType, 
        winding_type::WindingType
    }, 
    kernel::storage::storage::Storage
};
///
/// Storage of [the hoisting rope initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct HoistRopeInitialCtx {
    /// value [winding type](design\docs\algorithm\part01\initial_data.md)
    pub winding_type: WindingType,
    /// vector of data base hoisting ropes
    pub hoisting_ropes: Vec<HoistingRope>,
    /// value of [hoisting rope type](docs\$catalogsPurchasedEquipment.xlsx)
    pub rope_type: RopeType,
    /// vector of [hoisting rope diameters](docs\$catalogsPurchasedEquipment.xlsx)
    pub rope_diameters: Vec<f64>,
    /// value of [hoisting rope marked group count](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub rope_count: u8,
    /// value of [hoisting rope durability class](docs\$catalogsPurchasedEquipment.xlsx)
    pub rope_durability_class: RopeDurabilityClass,
} 
//
//
impl HoistRopeInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("HoistRopeInitialCtx".to_string());
        Ok(
            Self {
                winding_type: serde_json::from_value::<WindingType>(
                    storage_initial_data.load("test.user_characteristics.winding_type")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                hoisting_ropes: serde_json::from_value::<Vec<HoistingRope>>(
                    storage_initial_data.load("test.constructions.hoisting_ropes")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                rope_type: serde_json::from_value::<RopeType>(
                    storage_initial_data.load("test.user_characteristics.hoisting_rope_type")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                rope_diameters: serde_json::from_value::<Vec<f64>>(
                    storage_initial_data.load("test.user_characteristics.hoisting_rope_diameters")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                rope_count: serde_json::from_value::<u8>(
                    storage_initial_data.load("test.user_characteristics.hoisting_ropes_count")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                rope_durability_class: serde_json::from_value::<RopeDurabilityClass>(
                    storage_initial_data.load("test.user_characteristics.hoisting_rope_durability_class")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            }
        )
    }
}
