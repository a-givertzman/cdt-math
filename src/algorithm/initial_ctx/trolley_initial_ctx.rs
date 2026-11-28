use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::entities::trolley::{
        trolley_control_system::TrolleyControlSystem,
        trolley_group::TrolleyGroup,
        trolley_power_system::TrolleyPowerSystem
    }, 
    kernel::{
        dbgid::dbgid::DbgId, 
        storage::storage::Storage, 
    },
};
///
/// Storage of [the trolley running mechanism initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct TrolleyInitialCtx {
    /// value of slow [rated travelling trolley speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax: f64,
    /// value of slow [slow travelling trolley speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs: f64,
    /// type of [trolley group of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub group: TrolleyGroup,
    /// type of [trolley control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub control_system: TrolleyControlSystem,
    /// type of [trolley power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub power_system: TrolleyPowerSystem,
} 
//
//
impl TrolleyInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("TrolleyInitialCtx".to_string());
        Ok(
            Self {
                vhmax: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_trolley")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_trolley")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                group: serde_json::from_value::<TrolleyGroup>(
                    storage_initial_data.load("test.user_characteristics.trolley_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                control_system: serde_json::from_value::<TrolleyControlSystem>(
                    storage_initial_data.load("test.user_characteristics.trolley_control_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                power_system: serde_json::from_value::<TrolleyPowerSystem>(
                    storage_initial_data.load("test.user_characteristics.trolley_power_system")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
            }
        )
    }
}
