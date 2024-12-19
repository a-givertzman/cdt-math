use crate::kernel::dbgid::dbgid::DbgId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
///
/// Класс, для хранения информации о крюке
/// - 'hook_type' - тип крюка
/// - 'standard' - ГОСТ номер
/// - 'sequence_number' - порядковый номер
/// - 'capacity_M1' - грузоподъёмность крюка при M1 нагрузке механизма подъема
/// - 'shank_diameter' - диаметр хвостовика крюка под подшипник
/// - 'weight' - масса крюка
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hook {
    dbgid: DbgId,
    hook_type: String,
    standard: String,
    sequence_number: u8,
    capacity_m1: f64,
    shank_diameter: f64,
    weight: f64,
}
//
//
impl Hook{
    ///
    /// Конструктор класса Hook
    pub fn new() -> Self{
        Self { 
                dbgid: DbgId("Hook".to_string()),
                hook_type: String::new(),
                standard: String::new(), 
                sequence_number: 0,
                capacity_m1: 0.0, 
                shank_diameter: 0.0, 
                weight: 0.0,
        }
    }
    ///
    /// Метод вывода информации о крюке в консоль
    pub fn paint(&self) {
        log::debug!("{:?}", self);
    }
    ///
    /// Метод заполнения информации о крюке
    /// - 'value' - объект структуры serde_json:Value
    pub fn from_value(value: Value) -> Option<Self> {
        match serde_json::from_value::<Hook>(value) {
            Ok(hook) => Some(hook),
            Err(e) => {
                log::error!("Failed to deserialize Hook from Value: {}", e);
                None
            }
        }
    }
}