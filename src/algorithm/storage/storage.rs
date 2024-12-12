use serde::Serialize;
use serde_json::Value;
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
    hash: HashMap<PathBuf,Value>
}
//
//
impl Storage {
    ///
    /// Конструктор класса
    /// - `file_path` - путь к файлу JSON
    pub fn new() -> Self {
        Storage {
            hash: HashMap::new(),
            dbgid: DbgId("Storage".to_string()),
        }
    }
    ///
    /// Метод получения данных из json файла
    /// - 'key' - ключ к данным
    pub fn load(&mut self, key: &str) -> Result<Value, StrErr> {
        let key = key.trim_start_matches('.').to_owned();
        if key.is_empty() {
            return Err(StrErr(format!("{}.load | Key can't be empty", self.dbgid)));
        } else if key.contains("..") || key.contains('\\') {
            return Err(StrErr(format!("{}.load | Invalid structure of key", self.dbgid)));
        }
        match self.hash.get(&PathBuf::from(&key)) {
            Some(value) => return Ok(value.clone()),
            None => {
                let file = OpenOptions::new()
                    .read(true)
                    .open(&key)
                    .map_err(|err| StrErr(format!("{}.load | Failed to open file: {}", self.dbgid, err)))?;
                match serde_json::from_reader(file) {
                    Ok(json_value) => {
                        self.hash.insert(key.clone().into(), json_value.clone());
                        Ok(json_value)
                    }
                    Err(err) => StrErr(format!("{}.load | Invalid JSON: {}", self.dbgid, err)),
                }
            }
        }
    }
    ///
    /// Метод установления значения по указанному пути
    /// - `key` - ключ к данным
    /// - `value` - значение для хранения
    pub fn store(&mut self, key: &str, value: impl Serialize) -> Result<(), StrErr> {
        let key = key.trim_start_matches('.').to_owned();
        if key.is_empty(){
            return Err(StrErr(format!("{}.store | Key can't be empty",self.dbgid)))
        } else if key.contains("..") || key.contains('\\'){
            return Err(StrErr(format!("{}.store | Invalid structure of key",self.dbgid)))
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(key)
            .map_err(|err| StrErr(format!("{}.store | Error: {}", self.dbgid, err)))?;
        match serde_json::to_writer_pretty(file, &value) {
            Ok(_) => Ok(()),
            Err(err) => Err(StrErr(format!("{}.store | Parse error: {}", self.dbgid, err))),
        }
    }
}