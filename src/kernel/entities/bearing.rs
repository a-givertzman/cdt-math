use serde::{Deserialize, Serialize};
///
/// Класс, для хранения информации о подшипнике
/// - 'name' - имя подшипника
/// - 'static_capacity' - статическая грузоподъемность
/// - 'outer_diameter' - наружный диаметр
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bearing {
    pub name: String,
    static_capacity: f64,
    outer_diameter: f64,
}