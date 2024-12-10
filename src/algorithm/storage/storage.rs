use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use crate::kernel::dbgid::dbgid::DbgId;
use crate::kernel::str_err::str_err::StrErr;
///
/// Класс, для хранения данных из json файла
/// - 'file_path' - путь к файлу json
/// - 'hash' - кэш-хранилище
pub struct Storage {
    pub dbgid: DbgId,
    path: PathBuf,
    hash: HashMap<String,Value>
}
//
//
impl Storage {
    ///
    /// Конструктор класса
    /// - `file_path` - путь к файлу JSON
    pub fn new(file_path: &str) -> Self {
        Storage {
            hash: HashMap::new(),
            dbgid: DbgId("Storage".to_string()),
            path: file_path.to_string(),
        }
    }   
    ///
    /// Метод сохранения изменений в файл json
    /// - 'data' - информация, которую надо сохранить
    fn save(&self, data: Value) -> Result<(), StrErr> {
        match serde_json::to_string_pretty(&data) {
            Ok(content) => {
                match fs::write(&self.path,content) {
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
    pub fn load(&mut self, key: &str) -> Result<Value, StrErr> {
        if self.hash.contains_key(key) {
            match self.hash.get(key) {
                Some(value) => Ok(value.clone()),
                None => Err(StrErr(format!("{}.load | Empty value on this path",self.dbgid))),
            }
        } else {
            //Считываем данные из json данные
            let json_data = fs::read_to_string(&self.path)
                .map_err(|e| format!("{}.load | Failed to read file: {}", self.dbgid,e))?;
            // Преобразуем в serde json Value
            let json_value: Value = serde_json::from_str(&json_data)
                .map_err(|e| format!("{}.load | Invalid JSON: {}", self.dbgid,e))?;
            // Обработка и добавление
            match json_value {
                Value::Object(map) => {
                    let result: Value = map.get(key)
                        .cloned()
                        .ok_or_else(|| format!("{}.load | Key '{}' not found", self.dbgid,key))?;
                    self.hash.insert(key.to_owned(), result.clone());
                    Ok(result.clone())
                },
                _ => Err(StrErr(format!("{}.load | JSON is not an object",self.dbgid))),
            }
        }
    }
    ///
    /// Метод установления значения по указанному пути
    /// - `key` - ключ к данным
    /// - `value` - значение для хранения
    pub fn store(&mut self, key: &str, value: impl Serialize) -> Result<(), StrErr> {
        let key = key.to_string();
        let key = match key.get(0) {
            Some(first) => match first {
                '/' => {
                    key.remove(0);
                    key
                }
                _ => key
            }
            None => return Err(StrErr(format!("{}.store | Key can't be empty", self.dbgid))),
        };
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.join(key));
        match file {
            Ok(file) => {
                match serde_json::to_writer_pretty(file, &value) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(StrErr(format!("{}.store | Error: {}", self.dbgid, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.store | Error: {}", self.dbgid, err))),
        }
    }
}