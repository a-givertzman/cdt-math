use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;
use crate::Param_to_compare;
use std::io;
///
/// Класс, реализующий выбор крюков, относительно характеристик пользователя
///  hook - вектор в котором хранится, подходящий крюк
///  bearing - имя подходящего подшипника для крюка (hook)
/// 
pub struct Hook{
    pub hook: Vec<String>,
    pub bearing: String,
    hook_type: String,
    m_to_lift: f64,
    work_type: String,
    fmg: f64,
}
//
//
//
impl Hook{
    ///
    /// Метод создание нового экземпляра класса Hook
    /// _param_comp - экземпляр класса Param_to_compare, в котором хранятся готовые характеристики для выбора крюка
    /// 
    pub fn new(_param_comp: Param_to_compare) -> Self {
        Self {
            hook: Vec::new(),
            bearing: String::new(),
            hook_type: _param_comp._hook_type,
            m_to_lift: _param_comp._m_to_lift,
            work_type: _param_comp._m_work_type,
            fmg: _param_comp._fmg,
        }
    }
    ///
    /// Метод для нахождения крюков и подшинпиков
    /// hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    /// 
    pub fn eval(&mut self,hooks_storage: &mut Storage){
        //Выбор крюка
        let res_hook = Self::hook_select(Self::weight_check(&self.work_type, self.m_to_lift,&self.hook_type,hooks_storage));

        //Выбор подшипника
        let res_bearing: String = Self::bearing_select(Self::bearing_check(self.fmg, &self.hook_type, hooks_storage, &res_hook));

        self.hook = res_hook;
        self.bearing = res_bearing;

    }
    ///
    /// Метод выбора крюков из предложенных
    /// hooks - вектор векторов в котором лежат все подходящие крюки
    /// 
    fn hook_select(hooks: Vec<Vec<String>>) -> Vec<String> {
        println!("Which one do you choose?");
        let mut counter: usize = 0;
    
        // Печать вариантов выбора
        for value in hooks.iter() {
            println!("{} - {:?}", counter, value);
            counter += 1;
        }
    
        let mut user_select = String::new();
        
        // Чтение выбора пользователя
        match io::stdin().read_line(&mut user_select) {
            Ok(_) => {},
            Err(e) => {
                println!("Input error! {}", e);
                return Vec::new(); // Возврат пустого вектора в случае ошибки
            }
        }
    
        // Преобразование ввода пользователя в число
        match user_select.trim().parse::<usize>() {
            Ok(index) if index < hooks.len() => hooks[index].clone(),
            _ => {
                println!("Invalid selection!");
                Vec::new() // Возврат пустого вектора при неверном вводе
            }
        }
    }
    ///
    /// Метод выбора подшипников из предложенных
    /// bearings - вектор в котором лежат все подходящие подшипники
    /// 
    fn bearing_select(bearings: Vec<String>) -> String{
        println!("{:?}",bearings);

        if bearings.len()!=0{
            println!("Which bearing do you choose?");
            let mut counter: usize = 0;
            
            // Печать вариантов выбора
            for value in bearings.iter() {
                println!("{} - {:?}", counter, value);
                counter += 1;
            }
            
            let mut user_select = String::new();

            // Чтение выбора пользователя
            match io::stdin().read_line(&mut user_select) {
                Ok(_) => {},
                Err(e) => {
                    println!("Input error! {}", e);
                    return String::new(); // Возврат пустого вектора в случае ошибки
                }
            }

            bearings[1].clone()
        }
        else{
            println!("There is no right bearing for your hook");
            String::new()
        }
    }
    ///
    /// Метод выбора крюков, где всевозможные варианты фильтруются по условию грузоподъемности 
    /// bearings - вектор в котором лежат все подходящие подшипники
    /// work_type - тип работы механизма подъема
    /// m_to_lift - масса на крюке
    /// hook_type - тип крюка
    /// hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    ///  
    pub fn weight_check(work_type: &String, m_to_lift: f64, hook_type: &String, hooks_storage: &mut Storage) -> Vec<Vec<String>> {
        let mut res_hooks: Vec<Vec<String>> = Vec::new();
        if let Some(value) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО", hook_type.trim())) {
            if let Value::NextMap(map) = value {
                for (key_iso, _) in map {
                    let mut tmp_hooks: Vec<String> = Vec::new();
                    if let Some(vall) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/грузоподъемность/{}/", hook_type.trim(), key_iso, work_type.trim())) {
                        if let Value::Data(datt) = vall {
                            if m_to_lift <= *datt {
                                tmp_hooks.push(key_iso.to_string());
                                tmp_hooks.push(hook_type.to_string());
                                tmp_hooks.push(work_type.to_string());
                                tmp_hooks.push(format!("{}",*datt));
                                if let Some(v) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/масса заготовки/", hook_type.trim(), key_iso)){
                                    if let Value::Data(da) = v {
                                        tmp_hooks.push(format!("{}",*da));
                                    }
                                }
                            }
                        }
                    }
                    if !tmp_hooks.is_empty() {
                        res_hooks.push(tmp_hooks);
                    }
                }
            }
        } else {
            println!("Path not found for hook selection.");
        }
        // Сортируем `res_hooks` по первому элементу каждого вектора, преобразованному в число.
        res_hooks.sort_by(|a, b| {
            a[0].parse::<i32>().unwrap_or(0).cmp(&b[0].parse::<i32>().unwrap_or(0))
        });
        res_hooks
    }
    ///
    /// Метод выбора подшипников, где всевозможные варианты фильтруются по условию совпдания диаметров 
    /// hook_type - тип крюка
    /// fmg - сила тяжести, действующая на крюк
    /// hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    /// res_hooks - вектор, в котором хранятся все подходящие крюки
    /// 
    pub fn bearing_check(fmg: f64, hook_type: &String,hooks_storage: &mut Storage,res_hooks: &Vec<String>) -> Vec<String> {
        let mut tmp_bearings: Vec<String> = Vec::new();
    
        if let Some(value) = hooks_storage.get("конструкции/подшипники/название") {
            if let Value::NextMap(bearings_map) = value {
                for (bearing_name, _) in bearings_map {
                    if let Some(Value::Data(static_load)) = hooks_storage.get(
                        &format!("конструкции/подшипники/название/{}/статическая грузоподъемность/", bearing_name),
                    ) {
                        if fmg <= *static_load {
                            for res_hook in res_hooks {
                                if let Some(Value::Data(hook_diameter)) = hooks_storage.get(
                                    &format!(
                                        "конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/",
                                        hook_type.trim(),
                                        res_hook
                                    ),
                                ) {
                                    if let Some(Value::Data(bearing_outer_diameter)) = hooks_storage.get(
                                        &format!("конструкции/подшипники/название/{}/наружный диаметр/", bearing_name),
                                    ) {
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
                .collect::<String>()            // Собираем ее в строку
                .parse::<i32>()                 // Преобразуем строку в число
                .unwrap_or(0)                   // Обрабатываем ошибку преобразования, если будет
        });
        tmp_bearings
    }
    
}