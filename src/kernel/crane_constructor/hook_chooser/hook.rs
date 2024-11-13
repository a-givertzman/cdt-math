use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;
use super::all_hooks::{self, AllHooks};
use std::io;
pub struct Hook{
    pub dbgid: String,
    pub all_hooks: Vec<Vec<String>>,
    pub hook: Vec<String>,
    pub bearing: String,
    pub cargo_name: String,
    pub cargo_weight: f64,
    pub summary_weight: f64,
    pub good_weight: f64,
    pub hook_type: String,
    pub m_to_lift: f64,
    pub work_type: String,
    pub fmg: f64,
}
//
//
//
impl Hook{
    ///
    /// Метод создание нового экземпляра класса Hook
    /// - all_hooks - экземпляр класса AllHooks, в котором хранятся готовые крюки для выбора нужного
    /// 
    pub fn new(all_hooks: AllHooks) -> Self{
        Self {
            dbgid: String::from(format!("{}/Hook",all_hooks.dbgid)),
            cargo_weight: all_hooks.cargo_weight,
            summary_weight: 0.0,
            good_weight: 0.0,
            cargo_name: all_hooks.cargo_name,
            all_hooks: all_hooks.hooks, 
            hook: Vec::new(), 
            bearing: String::new(), 
            hook_type: all_hooks.hook_type, 
            m_to_lift: all_hooks.m_to_lift, 
            work_type: all_hooks.work_type, 
            fmg: all_hooks.fmg 
        }
    }
    ///
    /// Метод для нахождения крюка и подшипника из списка подходящих
    /// - hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    /// 
    pub fn eval(&mut self,hook_storage: &mut Storage){
        self.hook = self.hook_select(self.all_hooks.clone());
        // Расчет массы грузозахватного органа
        let mut hook_weight = 0.0;
        match self.hook[4].parse::<f64>() {
            Ok(value) => {
                hook_weight = value;
            }
            Err(e) => {
                println!("Failed to parse string: {}", e);
            }
        }
        let  (mut tmp_summary_weight,mut tmp_good_weight) = (hook_weight,0.0);
        
        if self.cargo_name.is_empty(){
            (tmp_summary_weight, tmp_good_weight) = Self::cargo_weights(self.m_to_lift,self.cargo_weight, hook_weight);
        }
        self.summary_weight = tmp_summary_weight;
        self.good_weight = tmp_good_weight;
    }
    /// 
    /// Метод для вычисления суммарной и полезной масс доп грузозахватного органа
    /// - m_to_lift - масса на крюке
    /// - cargo_weight - масса доп грузозахватного органа
    /// - hook_weight  - масса, выбранного пользователем крюка
    /// 
    pub fn cargo_weights(m_to_lift: f64, cargo_weight: f64,hook_weight: f64) -> (f64,f64){
        let summary_weight = hook_weight + cargo_weight;
        let good_weight = m_to_lift - cargo_weight;
    
        (summary_weight,good_weight)
    }
    ///
    /// Метод выбора крюков из предложенных
    /// - hooks - вектор векторов в котором лежат все подходящие крюки
    /// 
    fn hook_select(&self,hooks: Vec<Vec<String>>) -> Vec<String> {
        log::debug!("{}.hook_select | Which one do you choose?", self.dbgid);
        let mut counter: usize = 0;
    
        // Печать вариантов выбора
        for value in hooks.iter() {
            log::debug!("{}.hook_select | Hook {}: {:?}", self.dbgid, counter,value);
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
    
}