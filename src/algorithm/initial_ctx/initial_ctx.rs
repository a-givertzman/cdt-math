use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::entities::{
        alt_lift_device::AltLiftDevice, basic_crane_control::BasicCraneControl, bearing::Bearing, bridge_control_system::BridgeControlSystem, bridge_drive_diagram::BridgeDriveDiagram, bridge_drive_system::BridgeDriveSystem, bridge_movement_duration::BridgeMovementDuration, cab_location::CabLocation, crane_climatic_category::CraneClimaticCategory, crane_drive_group::CraneDriveGroup, crane_duty_class::CraneDutyClass, crane_power_system::CranePowerSystem, crane_purpose::CranePurpose, crane_wind_area::CraneWindArea, crane_work_area_type::CraneWorkArea, explosion_fire_save_crane_purpose::ExplosionFireSaveCranePurpose, hoist::hoist::Hoist, hoist_control_system::HoistControlSystem, hoist_group::HoistGroup, hoisting_rope::{
            hoisting_rope::HoistingRope, 
            rope_durability_class::RopeDurabilityClass, 
            rope_type::RopeType
        }, hook::HookBlock, lifted_load::LiftedLoad, lifting_class::LiftClass, lifting_duration::LiftingDuration, lifting_mechanism_drive_type::LiftingMechanismDriveType, loading_combination::LoadingCombination, trolley_control_system::TrolleyControlSystem, trolley_group::TrolleyGroup, trolley_power_system::TrolleyPowerSystem, winding_type::WindingType
    },
    kernel::{
        dbgid::dbgid::DbgId, 
        storage::storage::Storage, 
    },
};
///
/// Storage of [initial data](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone)]
pub struct InitialCtx {
    // dbgid: DbgId,
    /// value of [loading capacity](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub load: f64,
    /// value of [height of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifting_height: f64,
    /// value of nominal [rated travelling hoist speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax_hoist: f64,
    /// value of slow [slow travelling hoist speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs_hoist: f64,
    /// type of [hoisting group of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub hoist_group: HoistGroup,
    /// [work duration of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifting_duration: LiftingDuration,
    /// type of [hoisting control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub hoist_control_system: HoistControlSystem,
    /// where store initial [lifting mechanism drive type](design\docs\algorithm\part01\initial_data.md)
    pub driver_type: LiftingMechanismDriveType,
    /// type of [lifted load](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifted_load: LiftedLoad,
    /// value of slow [rated travelling trolley speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax_trolley: f64,
    /// value of slow [slow travelling trolley speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs_trolley: f64,
    /// type of [trolley group of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub trolley_group: TrolleyGroup,
    /// type of [trolley control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub trolley_control_system: TrolleyControlSystem,
    /// type of [trolley power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub trolley_power_system: TrolleyPowerSystem,
    /// value of nominal [rated travelling bridge speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax_bridge: f64,
    /// value of slow [slow travelling bridge speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs_bridge: f64,
    /// type of [crane drive group of lifting mechanism] (design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_drive_group: CraneDriveGroup,
    /// [work duration of bridge lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub bridge_movement_duration: BridgeMovementDuration,
    /// type of [bridge control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub bridge_control_system: BridgeControlSystem,
    /// type of [crane power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_power_system: CranePowerSystem,
    /// [bridge drive type system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub bridge_drive_system: BridgeDriveSystem,
    /// [bridge drive diagram](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub bridge_drive_diagram: BridgeDriveDiagram,
    /// [bridge control system of synchronous movement](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub bridge_synchronous_movement: bool,
    /// [type crane rail](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_rail: Option<String>,
    /// [crane rail length](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub rail_length: Option<f64>,
    /// [crane purpose](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_purpose: CranePurpose,
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
    /// [crane span](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_span: f64,
    /// [left edge approach lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub left_edge_approach_lifting_device: Option<f64>,
    /// [right edge approach lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub right_edge_approach_lifting_device: Option<f64>,
    /// [vertical distance from crane rail to lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vertical_distance_from_crane_rail_to_lifting_device: Option<f64>,
    /// [max crane base](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_crane_base: f64,
    /// [maximum vertical distance from the crane rail to the top of the crane](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub maximum_vertical_distance_from_the_crane_rail_to_the_top_of_the_crane: Option<f64>,
    /// [max width base](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_width_crane: f64,
    /// where store initial [loading combination](design\docs\algorithm\part01\initial_data.md)
    pub load_comb: LoadingCombination,
    /// value of [lifting class](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub lift_class: LiftClass,
    /// vector of data base hook blocks
    pub hooks: Vec<HookBlock>,
    /// vector of data base hoist's
    pub hoists: Vec<Hoist>,
    /// vector of data base bearings
    pub bearings: Vec<Bearing>,
    /// user [alternative lifting device](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub user_alt_lift_device: Option<AltLiftDevice>,
    /// value [deflection blocks count](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub deflect_blocks_count: f64,
    /// value [winding type](design\docs\algorithm\part01\initial_data.md)
    pub winding_type: WindingType,
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
        Ok(
            Self {
                load: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.load")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                lifting_height: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.lifting_height")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhmax_hoist: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_hoist")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs_hoist: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_hoist")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                hoist_group: serde_json::from_value::<HoistGroup>(
                    storage_initial_data.load("test.user_characteristics.hoist_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                lifting_duration: serde_json::from_value::<LiftingDuration>(
                    storage_initial_data.load("test.user_characteristics.lifting_duration")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                hoist_control_system: serde_json::from_value::<HoistControlSystem>(
                    storage_initial_data.load("test.user_characteristics.hoist_control_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                driver_type: serde_json::from_value::<LiftingMechanismDriveType>(
                    storage_initial_data.load("test.user_characteristics.driver_type")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                lifted_load: serde_json::from_value::<LiftedLoad>(
                    storage_initial_data.load("test.user_characteristics.lifted_load")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhmax_trolley: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_trolley")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs_trolley: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_trolley")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                trolley_group: serde_json::from_value::<TrolleyGroup>(
                    storage_initial_data.load("test.user_characteristics.trolley_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                trolley_control_system: serde_json::from_value::<TrolleyControlSystem>(
                    storage_initial_data.load("test.user_characteristics.trolley_control_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                trolley_power_system: serde_json::from_value::<TrolleyPowerSystem>(
                    storage_initial_data.load("test.user_characteristics.trolley_power_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhmax_bridge: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_bridge")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs_bridge: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_bridge")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_drive_group: serde_json::from_value::<CraneDriveGroup>(
                    storage_initial_data.load("test.user_characteristics.crane_drive_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_movement_duration: serde_json::from_value::<BridgeMovementDuration>(
                    storage_initial_data.load("test.user_characteristics.bridge_movement_duration")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_control_system: serde_json::from_value::<BridgeControlSystem>(
                    storage_initial_data.load("test.user_characteristics.bridge_control_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_power_system: serde_json::from_value::<CranePowerSystem>(
                    storage_initial_data.load("test.user_characteristics.crane_power_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_drive_system: serde_json::from_value::<BridgeDriveSystem>(
                    storage_initial_data.load("test.user_characteristics.bridge_drive_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_drive_diagram: serde_json::from_value::<BridgeDriveDiagram>(
                    storage_initial_data.load("test.user_characteristics.bridge_drive_diagram")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                bridge_synchronous_movement: serde_json::from_value::<bool>(
                    storage_initial_data.load("test.user_characteristics.bridge_synchronous_movement")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_rail: storage_initial_data
                    .load("test.user_characteristics.crane_rail")
                    .ok()
                    .and_then(|data| serde_json::from_value::<String>(data).ok()),
                rail_length: storage_initial_data
                    .load("test.user_characteristics.rail_length")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                crane_purpose: serde_json::from_value::<CranePurpose>(
                    storage_initial_data.load("test.user_characteristics.crane_purpose")?,
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
                crane_span: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.crane_span")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                left_edge_approach_lifting_device: storage_initial_data
                    .load("test.user_characteristics.left_edge_approach_lifting_device")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                right_edge_approach_lifting_device: storage_initial_data
                    .load("test.user_characteristics.right_edge_approach_lifting_device")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                vertical_distance_from_crane_rail_to_lifting_device: storage_initial_data
                    .load("test.user_characteristics.vertical_distance_from_crane_rail_to_lifting_device")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                max_crane_base: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.max_crane_base")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                maximum_vertical_distance_from_the_crane_rail_to_the_top_of_the_crane: storage_initial_data
                    .load("test.user_characteristics.maximum_vertical_distance_from_the_crane_rail_to_the_top_of_the_crane")
                    .ok()
                    .and_then(|data| serde_json::from_value::<f64>(data).ok()),
                max_width_crane: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.max_width_crane")?,
                )
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
                hoists: serde_json::from_value::<Vec<Hoist>>(
                    storage_initial_data.load("test.constructions.hoists")?,
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
            }
        )
    }
}
