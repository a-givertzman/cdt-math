use serde::Serialize;
use serde_json::{Value, Map};
use std::fs;
use std::path::Path;
use crate::kernel::dbgid::dbgid::DbgId;
use crate::kernel::str_err::str_err::StrErr;
///
/// Класс, для хранения данных из json файла
/// - 'file_path' - путь к файлу json
/// - 'data' - данные json файла
pub struct Storage {
    pub dbgid: DbgId,
    file_path: String,
    data: Value,
}
//
//
impl Storage {
    ///
    /// Конструктор класса
    /// - `file_path` - путь к файлу JSON
    pub fn new(file_path: &str) -> Self {
        let data = if Path::new(file_path).exists() {
            if let Ok(content) = fs::read_to_string(file_path) {
                serde_json::from_str(&content).unwrap_or_else(|_| Value::Object(Map::new()))
            } else {
                Value::Object(Map::new())
            }
        } else {
            Value::Object(Map::new())
        };
        Storage {
            dbgid: DbgId(format!("Storage")),
            file_path: file_path.to_string(),
            data,
        }
    }   
    ///
    /// Метод сохранения изменений в файл json
    fn save(&self) -> Result<(), StrErr> {
        match serde_json::to_string_pretty(&self.data){
            Ok(content) => {
                match fs::write(&self.file_path,content){
                    Ok(_) => return Ok(()),
                    Err(err) => return Err(StrErr(format!("{}.save | err: {:?}",self.dbgid,err))),
                }
            },
            Err(err) => return Err(StrErr(format!("{}.save | err: {:?}",self.dbgid,err))),
        }
    }
    ///
    /// Метод получения данных
    /// - 'key' - ключ к данным
    pub fn load(&self, key: &str) -> Result<Value, StrErr> {
        let keys = key.split('.').collect::<Vec<_>>();
        let mut current = &self.data;
        for key in keys {
            if let Value::Object(map) = current {
                current = map.get(key).ok_or_else(|| format!("Key '{}' not found", key))?;
            } else {
                return Err(StrErr(format!("{}.load | Invalid structure for key: {}",self.dbgid,key)));
            }
        }
        Ok(current.clone())
    }
    ///
    /// Метод установления значения по указанному пути
    /// - `key` - ключ к данным
    /// - `value` - значение для хранения
    pub fn store(&mut self, key: &str, value: impl Serialize) -> Result<(), StrErr> {
        let keys = key.split('.').collect::<Vec<_>>();
        let mut current = &mut self.data;
        match serde_json::to_value(value){
            Ok(value) => {
                for key in &keys[..keys.len() - 1] {
                    current = current.as_object_mut()
                        .ok_or("Object doesn't exist on this path")? 
                        .entry(key.to_string())
                        .or_insert(Value::Object(Map::new()));
                }
                if let Value::Object(map) = current {
                    map.insert(keys[keys.len() - 1].to_string(), value);
                    self.save() // Сохранение изменений
                } else {
                    Err(StrErr(format!("{}.store | Path leads to a non-object value: {}",self.dbgid,key)))
                }
            },
            Err(err) => Err(StrErr(format!("{}.store | err: {:?}",self.dbgid,err))),
        }
    }
}