use std::collections::HashMap;
///
/// Перечеслиние, для работы с хранилищем
///
pub enum Value {
    Data(f64),
    String(String),
    NextMap(HashMap<String, Value>),
}
//
//
//
impl Value {
    ///
    /// Метод, которой проверяет на вложенность таблицы
    ///
    pub fn as_nested(&self) -> Option<&HashMap<String, Value>> {
        if let Value::NextMap(map) = self {
            Some(map)
        } else {
            None
        }
    }
    ///
    /// Метод, которой проверяет на вложенность таблицы
    ///
    pub fn as_nested_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        if let Value::NextMap(map) = self {
            Some(map)
        } else {
            None
        }
    }
}