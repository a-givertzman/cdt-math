use crate::algorithm::user_select::user_select::UserSelect;
use serde::{Deserialize, Serialize};
///
/// Класс содержащий информацию о дополнительном грузозахватном органе
/// [reference to another load handing device documentation](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadHandingDevice{
    name: String,
    pub weight: f64,
}
//
impl LoadHandingDevice{
    ///
    /// Конструктор класса LoadHandingDevice
    pub fn new(user_select: &UserSelect) -> Self{
        Self{
            name: user_select.name_cargo_hand_device.clone(),
            weight: user_select.weight_cargo_hand_device,
        }
    }
}