use serde::{Deserialize, Serialize};

use super::{bearing::Bearing, hook::Hook};
///
/// Класс для хранения БД
/// - 'hooks' - таблица крюков
/// - 'bearings' - таблица подшипников
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipmentData {
    pub hooks: Vec<Hook>,
    pub bearings: Vec<Bearing>,
}