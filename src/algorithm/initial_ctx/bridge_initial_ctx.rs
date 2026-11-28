use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::entities::{
        bridge_control_system::BridgeControlSystem, 
        bridge_drive_diagram::BridgeDriveDiagram,
        bridge_drive_system::BridgeDriveSystem,
        bridge_movement_duration::BridgeMovementDuration,
        crane_drive_group::CraneDriveGroup, crane_power_system::CranePowerSystem,
    },
    kernel::{
        dbgid::dbgid::DbgId, 
        storage::storage::Storage, 
    },
};
///
/// Storage of [the bridge running mechanism initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct BridgeInitialCtx {
    /// value of nominal [rated travelling bridge speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax: f64,
    /// value of slow [slow travelling bridge speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs: f64,
    /// type of [crane drive group of lifting mechanism] (design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_drive_group: CraneDriveGroup,
    /// [work duration of bridge lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub movement_duration: BridgeMovementDuration,
    /// type of [bridge control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub control_system: BridgeControlSystem,
    /// type of [crane power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_power_system: CranePowerSystem,
    /// [bridge drive type system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub drive_system: BridgeDriveSystem,
    /// [bridge drive diagram](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub drive_diagram: BridgeDriveDiagram,
    /// [bridge control system of synchronous movement](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub synchronous_movement: bool,
    /// [type crane rail](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_rail: Option<String>,
    /// [crane rail length](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub rail_length: Option<f64>,
} 
//
//
impl BridgeInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("BridgeInitialCtx".to_string());
        Ok(
            Self {
                vhmax: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_bridge")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_bridge")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_drive_group: serde_json::from_value::<CraneDriveGroup>(
                    storage_initial_data.load("test.user_characteristics.crane_drive_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                movement_duration: serde_json::from_value::<BridgeMovementDuration>(
                    storage_initial_data.load("test.user_characteristics.bridge_movement_duration")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                control_system: serde_json::from_value::<BridgeControlSystem>(
                    storage_initial_data.load("test.user_characteristics.bridge_control_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                crane_power_system: serde_json::from_value::<CranePowerSystem>(
                    storage_initial_data.load("test.user_characteristics.crane_power_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                drive_system: serde_json::from_value::<BridgeDriveSystem>(
                    storage_initial_data.load("test.user_characteristics.bridge_drive_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                drive_diagram: serde_json::from_value::<BridgeDriveDiagram>(
                    storage_initial_data.load("test.user_characteristics.bridge_drive_diagram")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                synchronous_movement: serde_json::from_value::<bool>(
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
            }
        )
    }
}
