use std::io;

use crate::{algorithm::{hook_choose::{filter::hook_filter::HookFilter, hook::Hook}, storage::storage::Storage, user_select::user_select::UserSelect}, kernel::dbgid::dbgid::DbgId};
///
/// Класс который содержит крюк (экземпляр класса [Hook]) выбранный пользователем из предложенных
#[derive(Debug, Clone)]
pub struct UserHook{
    dbgid: DbgId,
    pub user_hook: Hook,
}
//
//
//
impl UserHook{
    ///
    /// Конструктор класса UserHook
    pub fn new() ->Self{
        Self {
            dbgid: DbgId(format!("UserHook")),
            user_hook: Hook::new(String::new(), String::new(), String::new(), 0.0, 0.0) }

    }
    ///
    /// Выбор крюка среди отфильтрованных
    pub fn eval(&mut self,user_select: &UserSelect ,hooks_storage: &mut Storage){
        let mut binding = HookFilter::new( user_select);
        let mut filtered_hooks = binding.filter(hooks_storage);

        

        for hook in filtered_hooks.iter(){
            log::debug!("{}", format!("{}.eval | {:?}", self.dbgid,hook.print()));
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