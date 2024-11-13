use crate::kernel::storage::storage::{Storage, Value};

use super::hook::Hook;


pub struct AllBearings{
    pub dbgid: String,
    fmg: f64,
    hook_type: String,
    res_hooks: Vec<String>,
    pub res_bearings: Vec<String>
}
//
//
//
impl AllBearings{
    ///
    /// Метод создания экземпляра класса AllBearings
    /// - hook - вектор подходящего крюка
    /// 
    pub fn new(hook: &Hook) -> Self{
        Self {  dbgid: String::from(format!("{}/AllBearings",hook.dbgid)),
                fmg: hook.fmg,
                hook_type: hook.hook_type.clone(),
                res_hooks: hook.hook.clone(),
                res_bearings: Vec::new(),
            }
    }
    ///
    /// Метод выбора всех подходящих подшипников для подходящего крюка
    /// - storage - экземпляр класса-храналища Storage с таблицей подшипников
    /// 
    pub fn eval(&mut self,storage: &Storage){
        self.res_bearings = Self::bearings_check(self.fmg, &self.hook_type, &storage, &self.res_hooks)
    }
    ///
    /// Метод выбора подшипников, где всевозможные варианты фильтруются по условию совпдания диаметров
    /// - hook_type - тип крюка
    /// - fmg - сила тяжести, действующая на крюк
    /// - hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    /// - res_hooks - вектор, в котором хранятся все подходящие крюки
    ///
    pub fn bearings_check(
        fmg: f64,
        hook_type: &String,
        hooks_storage: & Storage,
        res_hooks: &Vec<String>,
    ) -> Vec<String> {
        let mut tmp_bearings: Vec<String> = Vec::new();

        if let Some(value) = hooks_storage.get("конструкции/подшипники/название")
        {
            if let Value::NextMap(bearings_map) = value {
                for (bearing_name, _) in bearings_map {
                    if let Some(Value::Data(static_load)) = hooks_storage.get(&format!(
                        "конструкции/подшипники/название/{}/статическая грузоподъемность/",
                        bearing_name
                    )) {
                        if fmg <= *static_load {
                            for res_hook in res_hooks {
                                if let Some(Value::Data(hook_diameter)) =
                                    hooks_storage.get(&format!(
                                        "конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/",
                                        hook_type.trim(),
                                        res_hook
                                    ))
                                {
                                    if let Some(Value::Data(bearing_outer_diameter)) = hooks_storage
                                        .get(&format!(
                                            "конструкции/подшипники/название/{}/наружный диаметр/",
                                            bearing_name
                                        ))
                                    {
                                        if hook_diameter >= bearing_outer_diameter {
                                            tmp_bearings.push(bearing_name.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("Path not found for bearing selection.");
        }

        tmp_bearings.sort_by_key(|s| {
            s.chars()
                .take_while(|c| c.is_digit(10)) // Берем только числовую часть
                .collect::<String>() // Собираем ее в строку
                .parse::<i32>() // Преобразуем строку в число
                .unwrap_or(0) // Обрабатываем ошибку преобразования, если будет
        });
        tmp_bearings
    }
}
