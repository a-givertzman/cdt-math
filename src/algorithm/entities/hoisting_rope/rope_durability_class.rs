use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
///
/// Represents a [hoisting rope class durability](docs\catalogsPurchasedEquipment.xlsx)
pub enum RopeDurabilityClass {
    #[default]
    C1370,
    C1470,
    C1570,
    C1670,
    C1770,
    C1870,
    C1970,
    C2170,
}