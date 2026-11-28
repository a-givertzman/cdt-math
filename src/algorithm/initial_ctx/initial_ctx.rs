use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::{
        entities::{
            bearing::Bearing,
            hook::HookBlock,
            lifting_class::LiftClass,
            loading_combination::LoadingCombination,
        }, 
        initial_ctx::{
            bridge_initial_ctx::BridgeInitialCtx,
            general_crane_initial_ctx::GeneralCraneInitialCtx,
            hoist_initial_ctx::HoistInitialCtx,
            hoist_rope_initial_ctx::HoistRopeInitialCtx,
            overall_crane_initial_ctx::OverallCraneInitialCtx,
            trolley_initial_ctx::TrolleyInitialCtx
        }
    }, 
    kernel::{
        dbgid::dbgid::DbgId, 
        storage::storage::Storage, 
    },
};
///
/// Storage of [initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct InitialCtx {
    // dbgid: DbgId,
    /// storage of [hoist initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub hoist_initial: HoistInitialCtx,
    /// storage of [the trolley running mechanism initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub trolley_initial: TrolleyInitialCtx,
    /// storage of [the bridge running mechanism initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub bridge_initial: BridgeInitialCtx,
    /// storage of [general crane initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub general_crane_initial: GeneralCraneInitialCtx,
    /// storage of [the overall crane initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub overall_crane_initial: OverallCraneInitialCtx,
    /// storage of [the hoisting rope initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub hoist_rope_initial: HoistRopeInitialCtx,
    /// where store initial [loading combination](design/docs/algorithm/part02/chapter_01_choose_hook.md)
    pub load_comb: LoadingCombination,
    /// value of [lifting class](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub lift_class: LiftClass,
    /// value [deflection blocks count](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub deflect_blocks_count: f64,
    /// vector of data base hook blocks
    pub hooks: Vec<HookBlock>,
    /// vector of data base bearings
    pub bearings: Vec<Bearing>,
} 
//
//
impl InitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("InitialCtx".to_string());
        Ok(
            Self {
                hoist_initial: HoistInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                trolley_initial: TrolleyInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_initial: BridgeInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                general_crane_initial: GeneralCraneInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                overall_crane_initial: OverallCraneInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                hoist_rope_initial: HoistRopeInitialCtx::new(storage_initial_data)
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                load_comb: serde_json::from_value::<LoadingCombination>(
                    storage_initial_data.load("test.user_characteristics.loading_combination")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                lift_class: serde_json::from_value::<LiftClass>(
                    storage_initial_data.load("test.user_characteristics.lifting_class")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                hooks: serde_json::from_value::<Vec<HookBlock>>(
                    storage_initial_data.load("test.constructions.hooks")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bearings: serde_json::from_value::<Vec<Bearing>>(
                    storage_initial_data.load("test.constructions.bearings")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                deflect_blocks_count: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.deflection_blocks_count")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            }
        )
    }
}
