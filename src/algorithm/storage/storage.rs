use serde::Serialize;
use serde_json::{Map, Value};
use std::fs;
use crate::kernel::dbgid::dbgid::DbgId;
use crate::kernel::str_err::str_err::StrErr;
///
/// Класс, для хранения данных из json файла
/// - 'file_path' - путь к файлу json
pub struct Storage {
    pub dbgid: DbgId,
    file_path: String,
}
//
//
impl Storage {
    ///
    /// Конструктор класса
    /// - `file_path` - путь к файлу JSON
    pub fn new(file_path: &str) -> Self {
        Storage {
            dbgid: DbgId("Storage".to_string()),
            file_path: file_path.to_string(),
        }
    }   
    ///
    /// Метод сохранения изменений в файл json
    /// - 'data' - информация, которую надо сохранить
    fn save(&self, data: Value) -> Result<(), StrErr> {
        match serde_json::to_string_pretty(&data) {
            Ok(content) => {
                match fs::write(&self.file_path,content) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StrErr(format!("{}.save | err: {:?}", self.dbgid, err))),
                }
            },
            Err(err) => Err(StrErr(format!("{}.save | err: {:?}", self.dbgid, err)))
        }
    }
    ///
    /// Метод получения данных из json файла
    /// - 'key' - ключ к данным
    pub fn load(&self, key: &str) -> Result<Value, StrErr> {
        //Считываем данные из json данные
        let json_data = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("{}.load | Failed to read file: {}", self.dbgid,e))?;
        // Преобразуем в serde json Value
        let json_value: Value = serde_json::from_str(&json_data)
            .map_err(|e| format!("{}.load | Invalid JSON: {}", self.dbgid,e))?;
        // Обработка и добавление
        match json_value {
            Value::Object(map) => Ok(map.get(key)
                .cloned()
                .ok_or_else(|| format!("{}.load | Key '{}' not found", self.dbgid,key))?),
            _ => Err(StrErr(format!("{}.load | JSON is not an object",self.dbgid))),
        }
    }
    ///
    /// Метод установления значения по указанному пути
    /// - `key` - ключ к данным
    /// - `value` - значение для хранения
    pub fn store(&mut self, key: &str, value: impl Serialize) -> Result<(), StrErr> {
        let keys: Vec<&str> = key.split('.').collect();
        //Считываем данные из json данные
        let json_data = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("{}.store | Failed to read file: {}", self.dbgid,e))?;
        // Преобразуем в serde json Value
        let mut json_value: Value = serde_json::from_str(&json_data)
            .map_err(|e| format!("{}.store | Invalid JSON: {}", self.dbgid,e))?;
        let mut current = &mut json_value;
        match serde_json::to_value(value){
            Ok(value) => {
                for key in &keys[..keys.len() - 1] {
                    current = current.as_object_mut()
                        .ok_or(format!("{}.store | Object doesn't exist on this path", self.dbgid))? 
                        .entry(key.to_string())
                        .or_insert(Value::Object(Map::new()));
                }
                if let Value::Object(map) = current {
                    map.insert(keys[keys.len() - 1].to_string(), value);
                    let _ = self.save(json_value); // Сохранение изменений
                } else {
                    return Err(StrErr(format!("{}.store | Path leads to a non-object value: {}",self.dbgid,key)));
                }
            }
            Err(err) => return Err(StrErr(format!("{}.store | Error: {}",self.dbgid,err)))
        }
        Ok(())
    }
}