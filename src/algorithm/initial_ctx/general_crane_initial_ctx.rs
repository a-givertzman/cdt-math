use sal_sync::services::entity::{dbg_id::DbgId, error::str_err::StrErr};
use crate::{algorithm::entities::general_crane::{
    basic_crane_control::BasicCraneControl,
    cab_location::CabLocation,
    crane_work_area_type::CraneWorkArea,
    crane_climatic_category::CraneClimaticCategory,
    crane_duty_class::CraneDutyClass,
    crane_purpose::CranePurpose,
    crane_type::CraneType,
    crane_wind_area::CraneWindArea,
    explosion_fire_save_crane_purpose::ExplosionFireSaveCranePurpose
}, kernel::storage::storage::Storage};
///
/// Storage of [general crane initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct GeneralCraneInitialCtx {
    pub r#type: CraneType,
    /// [crane purpose](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_purpose: CranePurpose,
    /// value [crane work area type](design\docs\algorithm\part01\initial_data.md)
    pub crane_work_area: CraneWorkArea,
    /// [explosion-fire-safe crane purpose](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub explosion_fire_safe_crane_purpose: ExplosionFireSaveCranePurpose,
    /// value [marking of fire/explosion hazardous operating environment](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub marking_of_fire_explosion_hazardous_operating_environment: bool,
    /// type of [crane duty class](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_duty_class: CraneDutyClass,
    /// [climatic design and placement category crane](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_climatic_category: CraneClimaticCategory,
    /// [crane wind area](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_wind_area: CraneWindArea,
    /// [max use temperature](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_use_temperature: f64,
    /// [min use temperature](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub min_use_temperature: f64,
    /// type of [basic crane control](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub basic_crane_control: BasicCraneControl,
    /// type of [cab location](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub cab_location: CabLocation,
    /// [identical hoists volume](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub identical_hoists_volume: f64,
    /// [max crane mass](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_crane_mass: Option<f64>,
    /// [max wheel load](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_wheel_load: Option<f64>,
} 
//
//
impl GeneralCraneInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("GeneralCraneInitialCtx".to_string());
        Ok(
            Self {
                r#type: serde_json::from_value::<CraneType>(
                    storage_initial_data.load("test.user_characteristics.crane_purpose")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_purpose: serde_json::from_value::<CranePurpose>(
                    storage_initial_data.load("test.user_characteristics.crane_purpose")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_work_area: serde_json::from_value::<CraneWorkArea>(
                    storage_initial_data.load("test.user_characteristics.crane_work_area_type")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                explosion_fire_safe_crane_purpose: serde_json::from_value::<ExplosionFireSaveCranePurpose>(
                    storage_initial_data.load("test.user_characteristics.explosion_fire_safe_crane_purpose")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                marking_of_fire_explosion_hazardous_operating_environment: serde_json::from_value::<bool>(
                    storage_initial_data.load("test.user_characteristics.mark_fire_exp_env")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_duty_class: serde_json::from_value::<CraneDutyClass>(
                    storage_initial_data.load("test.user_characteristics.crane_duty_class")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_climatic_category: serde_json::from_value::<CraneClimaticCategory>(
                    storage_initial_data.load("test.user_characteristics.crane_climatic_category")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_wind_area: serde_json::from_value::<CraneWindArea>(
                    storage_initial_data.load("test.user_characteristics.crane_wind_area")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                max_use_temperature: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.max_use_temperature")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                min_use_temperature: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.min_use_temperature")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                basic_crane_control: serde_json::from_value::<BasicCraneControl>(
                    storage_initial_data.load("test.user_characteristics.basic_crane_control")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                cab_location: serde_json::from_value::<CabLocation>(
                    storage_initial_data.load("test.user_characteristics.cab_location")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                identical_hoists_volume: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.identical_hoists_volume")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                max_crane_mass: storage_initial_data
                    .load("test.user_characteristics.max_crane_mass")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                max_wheel_load: storage_initial_data
                    .load("test.user_characteristics.max_wheel_load")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
            }
        )
    }
}
