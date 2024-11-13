use super::all_hooks::{self, AllHooks};
use crate::kernel::storage::storage::Storage;
use crate::kernel::storage::storage::Value;
use std::io;
///
/// Класс, реализующий выбор подходящего крюка, из списка подходящих
/// - all_hooks - таблица, в котором хранится все подхоядящие крюки
/// - hook - вектор в котором хранится, подходящий крюк
/// - hooks_type - имя типа крюка, выбранного пользователем
/// - m_to_lift - масса на крюке, введенная пользователем
/// - work_type - тип работа механизма работы
/// - fmg - сила тяжести, действующая на грузозахватный механизм
///
pub struct Hook {
    pub dbgid: String,
    pub all_hooks: Vec<Vec<String>>,
    pub hook: Vec<String>,
    pub bearing: String,
    pub hook_type: String,
    m_to_lift: f64,
    pub work_type: String,
    pub fmg: f64,
}
//
//
//
impl Hook {
    ///
    /// Метод создание нового экземпляра класса Hook
    /// - all_hooks - экземпляр класса AllHooks, в котором хранятся готовые крюки для выбора нужного
    ///
    pub fn new(all_hooks: AllHooks) -> Self {
        Self {
            dbgid: String::from(format!("{}/Hook",all_hooks.dbgid)),
            all_hooks: all_hooks.hooks,
            hook: Vec::new(),
            bearing: String::new(),
            hook_type: all_hooks.hook_type,
            m_to_lift: all_hooks.m_to_lift,
            work_type: all_hooks.work_type,
            fmg: all_hooks.fmg,
        }
    }
    ///
    /// Метод для нахождения крюка из списка подходящих
    /// - hooks_storage - экземпляр класса-хранилища Storage, в котором хранятся все крюки
    ///
    pub fn eval(&mut self, hook_storage: &mut Storage) {
        self.hook = self.hook_select(self.all_hooks.clone());
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
            Ok(_) => {}
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
