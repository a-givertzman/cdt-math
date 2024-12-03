use serde_json::{Value, Map};
use std::fs;
use std::path::Path;
use std::error::Error;
///
/// Класс, для хранения данных из json файла
/// - 'file_path' - путь к файлу json
/// - 'data' - данные json файла
pub struct Storage {
    file_path: String,
    data: Value,
}
//
//
impl Storage {
    ///
    /// Конструктор класса
    /// - 'file_path' - путь к файлу json
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let data = if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path)?;
            serde_json::from_str(&content)?
        } else {
            Value::Object(Map::new())
        };
        Ok(Storage {
            file_path: file_path.to_string(),
            data,
        })
    }
    ///
    /// Метод сохранения изменений в файл json
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
    ///
    /// Метод получения данных
    /// - 'path' - путь к данным
    pub fn get(&self, path: &str) -> Option<&Value> {
        let keys = path.split('.').collect::<Vec<_>>();
        let mut current = &self.data;
        for key in keys {
            if let Value::Object(map) = current {
                current = map.get(key)?;
            } else {
                return None;
            }
        }
        Some(current)
    }
    ///
    /// Метод установления значения по указанному пути
    /// - 'path' - путь к данным
    /// - 'value' - присваемое значение
    pub fn set(&mut self, path: &str, value: Value) -> Result<(), Box<dyn Error>> {
        let keys = path.split('.').collect::<Vec<_>>();
        let mut current = &mut self.data;
        for key in &keys[..keys.len() - 1] {
            current = current.as_object_mut()
                .ok_or("Object doesn't exist on this path")?
                .entry(key.to_string())
                .or_insert(Value::Object(Map::new()));
        }
        if let Value::Object(map) = current {
            map.insert(keys[keys.len() - 1].to_string(), value);
            self.save()
        } else {
            Err("Error to find value".into())
        }
    }
}