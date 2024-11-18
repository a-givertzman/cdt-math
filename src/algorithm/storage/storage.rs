use std::collections::HashMap;

use crate::kernel::entities::value::Value;
///
/// Класс, для хранение данных
/// storage - "база данных", в которой будут хранится таблицы
pub struct Storage {
    pub dbgid: String,
    storage: HashMap<String, Value>,
}
//
//
//
impl Storage {
    ///
    /// Метод создание нового экземпляра класса Storage
    pub fn new() -> Self {
        Storage {
            dbgid: String::from("Storage"),
            storage: HashMap::new(),
        }
    }
    ///
    /// Метод для установки ключа, в определенный путь
    /// - key_path - путь для нахождения нужной ячейки таблицы
    /// - value - значение которое присвоится ячейке
    pub fn set(&mut self, key_path: &str, value: Result<f64, String>) {
        let parts: Vec<&str> = key_path.split('/').collect();
        let mut current_map = self
            .storage
            .entry(parts[0].to_string())
            .or_insert_with(|| Value::NextMap(HashMap::new()))
            .as_nested_mut()
            .expect("Expected nested map");

        for part in &parts[1..parts.len() - 1] {
            current_map = current_map
                .entry(part.to_string())
                .or_insert_with(|| Value::NextMap(HashMap::new()))
                .as_nested_mut()
                .expect("Expected nested map");
        }

        // Вставляем значение в последний ключ
        match value {
            Ok(data) => {
                current_map.insert(parts.last().unwrap().to_string(), Value::Data(data));
            }
            Err(str) => {
                current_map.insert(parts.last().unwrap().to_string(), Value::String(str));
            }
        }
    }
    ///
    /// Метод для получения либо значения ячейки либо таблицы
    /// - key_path - путь для нахождения нунжого результата
    pub fn get(&self, key_path: &str) -> Option<&Value> {
        let parts: Vec<&str> = key_path.split('/').collect();
        let mut current_value = self.storage.get(parts[0])?;

        for part in &parts[1..] {
            match current_value.as_nested() {
                Some(next_map) => {
                    // Разыменовываем part для получения &str
                    current_value = next_map.get(*part)?;
                }
                None => return None, // Если текущий элемент не карта, возвращаем None
            }
        }

        Some(current_value)
    }
}
