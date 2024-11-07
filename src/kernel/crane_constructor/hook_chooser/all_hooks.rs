use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;
use crate::Param_to_compare;
///
/// Класс, реализующий выбор всех подходящихкрюков, относительно характеристик пользователя
/// - cargo_name - имя дополнительного грузозахватного органа
/// - cargo_weigt - масса дополнительного грузозахватного органа 
/// - hooks - вектор в котором хранится, подходящий крюк
/// - hooks_type - имя типа крюка, выбранного пользователем
/// - m_to_lift - масса на крюке, введенная пользователем
/// - work_type - тип работа механизма работы
/// - fmg - сила тяжести, действующая на грузозахватный механизм
/// 
pub struct AllHooks{
    pub dbgid: String,
    pub cargo_name: String,
    pub cargo_weight: f64,
    pub hooks: Vec<Vec<String>>,
    pub hook_type: String,
    pub m_to_lift: f64,
    pub work_type: String,
    pub fmg: f64,
}
//
//
//
impl AllHooks{
    ///
    /// Метод создание нового экземпляра класса Hook
    /// - _param_comp - экземпляр класса Param_to_compare, в котором хранятся готовые характеристики для выбора крюка
    /// 
    pub fn new(_param_comp: Param_to_compare) -> Self {
        Self {
            dbgid: String::from(format!("{}/AllHooks",_param_comp.dbgid)),
            cargo_weight: _param_comp.cargo_weight,
            cargo_name: _param_comp.cargo_name,
            hooks: Vec::new(),
            hook_type: _param_comp._hook_type,
            m_to_lift: _param_comp._m_to_lift,
            work_type: _param_comp._m_work_type,
            fmg: _param_comp._fmg,
        }
    }
    ///
    /// Метод для нахождения крюков и подшинпиков
    /// - hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    /// 
    pub fn eval(&mut self,hooks_storage: &mut Storage){
        //Выбор крюка
        self.hooks = Self::weight_check(&self.work_type, self.m_to_lift,&self.hook_type,hooks_storage);
    }
    ///
    /// Метод выбора крюков, где всевозможные варианты фильтруются по условию грузоподъемности 
    /// - work_type - тип работы механизма подъема
    /// - m_to_lift - масса на крюке
    /// - hook_type - тип крюка
    /// - hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
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
    
}