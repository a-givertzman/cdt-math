use sal_sync::services::entity::{
    dbg_id::DbgId, 
    error::str_err::StrErr
};
use crate::{
    algorithm::entities::hoist::{
        hoist_control_system::HoistControlSystem,
        hoist_group::HoistGroup,
        lifted_load::LiftedLoad,
        lifting_duration::LiftingDuration,
        lifting_mechanism_drive_type::LiftingMechanismDriveType,
        lifting_device::LiftingDevice,
    },
    kernel::storage::storage::Storage
};

///
///
/// Storage of [hoist initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct HoistInitialCtx {
    /// value of [loading capacity](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub load: f64,
    /// value of [height of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifting_height: f64,
    /// value [lifting device](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
    pub lifting_device: Option<LiftingDevice>,
    /// value of nominal [rated travelling hoist speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhmax: f64,
    /// value of slow [slow travelling hoist speed](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vhcs: f64,
    /// type of [hoisting group of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub group: HoistGroup,
    /// [work duration of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifting_duration: LiftingDuration,
    /// type of [hoisting control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub control_system: HoistControlSystem,
    /// where store initial [lifting mechanism drive type](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
    pub driver_type: LiftingMechanismDriveType,
    /// type of [lifted load](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub lifted_load: LiftedLoad,
    
} 
//
//
impl HoistInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("HoistInitialCtx".to_string());
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
                lifting_device: storage_initial_data
                    .load("test.user_characteristics.alternavite_lifting_device")
                    .ok()
                    .and_then(|data| serde_json::from_value::<LiftingDevice>(data).ok()),
                vhmax: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhmax_hoist")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                vhcs: serde_json::from_value::<f64>(
                    storage_initial_data.load("test.user_characteristics.vhcs_hoist")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                group: serde_json::from_value::<HoistGroup>(
                    storage_initial_data.load("test.user_characteristics.hoist_group")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                lifting_duration: serde_json::from_value::<LiftingDuration>(
                    storage_initial_data.load("test.user_characteristics.lifting_duration")?,
                )
                .map_err(|err| StrErr(format!("{}.new | Error {:?}", dbg, err)))?,
                control_system: serde_json::from_value::<HoistControlSystem>(
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
            }
        )
    }
}
