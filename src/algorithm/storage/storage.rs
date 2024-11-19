use crate::kernel::{dbgid::dbgid::DbgId, entities::{bearing::Bearing, equipment_data::EquipmentData, hook::Hook}};
///
/// Класс, реализующий хранилище БД
/// - 'equipment_data' - БД (экземпляр класса [EquipmentData])
pub struct Storage {
    pub dbgid: DbgId,
    equipment_data: Option<EquipmentData>,
}
//
impl Storage {
    ///
    /// Конструктор класса Storage
    pub fn new() -> Self {
        Storage {
            dbgid: DbgId(format!("Storage")),
            equipment_data: None,
        }
    }
    ///
    /// Метод чтения файла Json
    /// - 'file_path' - путь к файлу
    pub fn load_from_json(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(file_path)?;
        let data: EquipmentData = serde_json::from_str(&file)?;
        self.equipment_data = Some(data);
        Ok(())
    }
    ///
    /// Метод возвращения крюка (экземпляра класса [Hook])
    /// - 'index' - порядковый номер крюка
    pub fn get_hook(&self, index: usize) -> Option<&Hook> {
        self.equipment_data.as_ref()?.hooks.get(index)
    }
    ///
    /// Метод возвращения подшипника (экземпляра класса [Bearing])
    /// - 'name' - название подшипника
    pub fn get_bearing_by_name(&self, name: &str) -> Option<&Bearing> {
        self.equipment_data.as_ref()?.bearings.iter().find(|b| b.name == name)
    }
}