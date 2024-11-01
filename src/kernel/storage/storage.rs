use std::collections::HashMap;

pub enum Value {
    Data(f64),
    String(String),
    NextMap(HashMap<String, Value>),
}


pub struct Storage {
    storage: HashMap<String, Value>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key_path: &str, value: Result<f64,String>) {
        let parts: Vec<&str> = key_path.split('/').collect();
        let mut current_map = self.storage
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
            Ok(data) =>{
                current_map.insert(parts.last().unwrap().to_string(), Value::Data(data));
            }
            Err(str) => {
                current_map.insert(parts.last().unwrap().to_string(), Value::String(str));
            }
        }
        
    }

    

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

impl Value {
    pub fn as_nested(&self) -> Option<&HashMap<String, Value>> {
        if let Value::NextMap(map) = self {
            Some(map)
        } else {
            None
        }
    }

    pub fn as_nested_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        if let Value::NextMap(map) = self {
            Some(map)
        } else {
            None
        }
    }
}