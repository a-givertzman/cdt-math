use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
///
/// Represents a [hoisting rope core type](docs\catalogsPurchasedEquipment.xlsx)
pub enum RopeType {
    #[default]
    Metal,
    Synthetic
}