use serde::{Deserialize, Serialize};
///
/// Represents [lifting device](design/docs/algorithm_single_ginder_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct LiftingDevice {
    /// value of alternarive lifting device name 
    pub name: String,
    /// value of alternarive lifting device weight 
    pub weight: f64,
}