use sal_sync::services::entity::{dbg_id::DbgId, error::str_err::StrErr};
use crate::kernel::storage::storage::Storage;
///
/// Storage of [the overall crane initial data](design\docs\algorithm_single_ginger_overhead_crane\part01_initialization\chapter01_initialData\chapter01_initialData.md)
#[derive(Debug, Clone)]
pub struct OverallCraneInitialCtx {
    /// [crane span](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub crane_span: f64,
    /// [left edge approach lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub left_edge_approach_lifting_device: Option<f64>,
    /// [right edge approach lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub right_edge_approach_lifting_device: Option<f64>,
    /// [vertical distance from crane rail to lifting device](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub vertical_distance_from_crane_rail_to_lifting_device: Option<f64>,
    /// [maximum vertical distance from the crane rail to the top of the crane](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub maximum_vertical_distance_from_the_crane_rail_to_the_top_of_the_crane: Option<f64>,
    /// [max crane base](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_crane_base: f64,
    /// [max width base](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    pub max_width_crane: f64,
} 
//
//
impl OverallCraneInitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("OverallCraneInitialCtx".to_string());
        Ok(
            Self {
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
            }
        )
    }
}
