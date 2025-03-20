use sal_sync::services::entity::error::str_err::StrErr;

use crate::{
    algorithm::entities::{
        alt_lift_device::AltLiftDevice, bearing::Bearing, crane_work_area_type::CraneWorkArea, driver_type::DriverType, hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType}, hook::Hook, lifting_class::LiftClass, loading_combination::LoadingCombination, mechanism_work_type::MechanismWorkType, winding_type::WindingType
    },
    kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, },
};
///
/// Storage of [initial data](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone)]
pub struct InitialCtx {
    // dbgid: DbgId,
    /// where store initial [driver type](design\docs\algorithm\part01\initial_data.md)
    pub driver_type: DriverType,
    /// where store initial [loading combination](design\docs\algorithm\part01\initial_data.md)
    pub load_comb: LoadingCombination,
    /// value of nominal [lifting speed of the mechanism](design\docs\algorithm\part01\initial_data.md)
    pub vhmax: f64,
    /// value of slow [lifting speed of the mechanism](design\docs\algorithm\part01\initial_data.md)
    pub vhcs: f64,
    /// value of [lifting class](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub lift_class: LiftClass,
    /// vector of data base hooks
    pub hooks: Vec<Hook>,
    /// value of [loading capacity](design\docs\algorithm\part01\initial_data.md)
    pub load_capacity: f64,
    /// value of [mechanism work type](design\docs\algorithm\part01\initial_data.md)
    pub mechanism_work_type: MechanismWorkType,
    /// vector of data base bearings
    pub bearings: Vec<Bearing>,
    /// user [alternative lifting device](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub user_alt_lift_device: Option<AltLiftDevice>,
    /// value [deflection blocks count](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub deflect_blocks_count: f64,
    /// value [winding type](design\docs\algorithm\part01\initial_data.md)
    pub winding_type: WindingType,
    /// value [marking of fire/explosion hazardous environment](design\docs\algorithm\part01\initial_data.md)
    pub mark_fire_exp_env: bool,
    /// value [crane work area type](design\docs\algorithm\part01\initial_data.md)
    pub crane_work_area: CraneWorkArea,
    /// vector of data base hoisting ropes
    pub hoisting_ropes: Vec<HoistingRope>,
    /// value of [hoisting rope type](docs\$catalogsPurchasedEquipment.xlsx)
    pub hoist_rope_type: RopeType,
    /// vector of [hoisting rope diameters](docs\$catalogsPurchasedEquipment.xlsx)
    pub hoist_rope_diameters: Vec<f64>,
    /// value of [hoisting rope marked group count](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub hoist_rope_count: u8,
    /// value of [hoisting rope durability class](docs\$catalogsPurchasedEquipment.xlsx)
    pub hoist_rope_durability_class: RopeDurabilityClass,
} 
//
//
impl InitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("InitialCtx".to_string());
        Ok(Self {
            driver_type: serde_json::from_value::<DriverType>(
                storage_initial_data.load("test.user_characteristics.driver_type")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            load_comb: serde_json::from_value::<LoadingCombination>(
                storage_initial_data.load("test.user_characteristics.loading_combination")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            vhmax: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.vhmax")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            vhcs: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.vhcs")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            lift_class: serde_json::from_value::<LiftClass>(
                storage_initial_data.load("test.user_characteristics.lifting_class")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hooks: serde_json::from_value::<Vec<Hook>>(
                storage_initial_data.load("test.constructions.hooks")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            load_capacity: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.loading_capacity")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            mechanism_work_type: serde_json::from_value::<MechanismWorkType>(
                storage_initial_data.load("test.user_characteristics.mechanism_work_type")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            bearings: serde_json::from_value::<Vec<Bearing>>(
                storage_initial_data.load("test.constructions.bearings")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            user_alt_lift_device: storage_initial_data
                .load("test.user_characteristics.alternavite_lifting_device")
                .ok()
                .and_then(|data| serde_json::from_value::<AltLiftDevice>(data).ok()),
            deflect_blocks_count: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.deflection_blocks_count")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            winding_type: serde_json::from_value::<WindingType>(
                storage_initial_data.load("test.user_characteristics.winding_type")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            mark_fire_exp_env: serde_json::from_value::<bool>(
                storage_initial_data.load("test.user_characteristics.mark_fire_exp_env")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            crane_work_area: serde_json::from_value::<CraneWorkArea>(
                storage_initial_data.load("test.user_characteristics.crane_work_area_type")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hoisting_ropes: serde_json::from_value::<Vec<HoistingRope>>(
                storage_initial_data.load("test.constructions.hoisting_ropes")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hoist_rope_type: serde_json::from_value::<RopeType>(
                storage_initial_data.load("test.user_characteristics.hoisting_rope_type")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hoist_rope_diameters: serde_json::from_value::<Vec<f64>>(
                storage_initial_data.load("test.user_characteristics.hoisting_rope_diameters")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hoist_rope_count: serde_json::from_value::<u8>(
                storage_initial_data.load("test.user_characteristics.hoisting_ropes_count")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            hoist_rope_durability_class: serde_json::from_value::<RopeDurabilityClass>(
                storage_initial_data.load("test.user_characteristics.hoisting_rope_durability_class")?,
            )
            .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
        })
    }
}
