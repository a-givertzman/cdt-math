use std::io;

use crate::{algorithm::{bearing_choose::{bearing::Bearing, filter::bearing_filter::BearingFilter}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, force_of_gravity::force_of_gravity::ForceOfGravity, hook_choose::user_hook::user_hook::UserHook, storage::storage::Storage, user_select::user_select::UserSelect}, kernel::dbgid::dbgid::DbgId};
///
/// Класс который содержит крюк (экземпляр класса [Hook]) выбранный пользователем из предложенных
pub struct UserBearing{
    dbgid: DbgId,
    pub user_bearing: Bearing,
}
//
//
//
impl UserBearing{
    ///
    /// Конструктор класса UserHook
    pub fn new() ->Self{
        Self {
            dbgid: DbgId(format!("UserBearing")),
            user_bearing: Bearing::new(String::new(), 0.0) }

    }
    ///
    /// Выбор крюка среди отфильтрованных
    pub fn eval(&mut self,user_hook: &UserHook,user_select: &UserSelect ,hooks_storage: &mut Storage,dynamic_coefficient: DynamicCoefficient){
        let mut binding = BearingFilter::new(user_hook, ForceOfGravity::new(dynamic_coefficient, user_select.m_to_lift));
        let mut filtered_bearings = binding.filter(hooks_storage);

        if filtered_bearings.is_empty(){
            log::debug!("{}.eval | There is no bearing for your hook",self.dbgid);
        } else {
            for hook in filtered_bearings.iter(){
                log::debug!("{}", format!("{}.eval | {:?}", self.dbgid,hook.print()));
            }

            log::debug!("{}.eval | Please, select your bearing",self.dbgid);

            let mut user_select = String::new();

            // Чтение выбора пользователя
            match io::stdin().read_line(&mut user_select) {
                Ok(_) => {}
                _ => {
                
                }
            }

            // Преобразование ввода пользователя в число
            match user_select.trim().parse::<usize>() {
                Ok(index) => {
                    if index < filtered_bearings.len(){
                        self.user_bearing = filtered_bearings[index].to_owned();
                    }
                }
                _ =>{

                }
            }
        }   

    }

}