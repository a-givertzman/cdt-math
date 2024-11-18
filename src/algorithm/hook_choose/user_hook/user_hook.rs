use std::io;
use crate::{algorithm::{hook_choose::filter::hook_filter::HookFilter, storage::storage::Storage}, kernel::{dbgid::dbgid::DbgId, entities::{hook::Hook, mechanism_work_type::{self, MechanismWorkType}}}};
///
/// Класс который содержит крюк выбранный пользователем из предложенных
/// - 'user_hook' - крюк, выбранный пользователем (экземпляр класса [Hook])
#[derive(Debug, Clone)]
pub struct UserHook{
    dbgid: DbgId,
    pub user_hook: Hook,
}
//
impl UserHook{
    ///
    /// Конструктор класса UserHook
    pub fn new() ->Self{
        Self {
            dbgid: DbgId(format!("Hook/UserHook")),
            user_hook: Hook::new() }
    }
    ///
    /// Метод выбора крюка среди отфильтрованных
    /// - 'hook_type' - тип крюка
    /// - 'mechanism_work_type' - тип работы механизма подъема
    /// - 'm_to_lift' - масса на крюке
    /// - 'storage' - БД [Storage]
    pub fn eval(&mut self,hook_type: String, mechanism_work_type: MechanismWorkType, m_to_lift: f64, storage: &mut Storage){
        let mut binding = HookFilter::new();
        let filtered_hooks = binding.filter(storage, hook_type, mechanism_work_type, m_to_lift);
        for hook in filtered_hooks.iter(){
            log::debug!("{}", format!("{}.eval | {:?}", self.dbgid,hook.paint()));
        }
        let mut user_select = String::new();
        log::debug!("{}.eval | Please, select your hook",self.dbgid);
        // Чтение выбора пользователя
        match io::stdin().read_line(&mut user_select) {
            Ok(_) => {}
            _ => {
               
            }
        }
        // Преобразование ввода пользователя в число
        match user_select.trim().parse::<usize>() {
            Ok(index) => {
                if index < filtered_hooks.len(){
                    self.user_hook = filtered_hooks[index].to_owned();
                }
            }
            _ =>{

            }
        }

    }

}