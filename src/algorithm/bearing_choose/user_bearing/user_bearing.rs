use std::io;

use crate::{algorithm::{bearing_choose::filter::bearing_filter::BearingFilter, hook_choose::user_hook::user_hook::UserHook, storage::storage::Storage}, kernel::{dbgid::dbgid::DbgId, entities::{bearing::Bearing, driver_type::DriverType, lift_class::LiftClass, load_combination::LoadCombination}}};
///
/// Класс который содержит крюк (экземпляр класса [Hook]) выбранный пользователем из предложенных
pub struct UserBearing{
    dbgid: DbgId,
    pub user_bearing: Bearing,
}
//
impl UserBearing{
    ///
    /// Конструктор класса UserHook
    pub fn new() ->Self{
        Self {
            dbgid: DbgId(format!("Bearing/UserBearing")),
            user_bearing: Bearing::new() }

    }
    ///
    /// Выбор крюка среди отфильтрованных
    /// [reference to bearing filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'storage' - БД [Storage]
    /// - 'user_hook' - крюк выбранный пользователем (экземпляр класса [UserHook])
    /// - 'm_to_lift' - масса на крюке
    /// - 'lift_class' - класс подъема (enum [LiftClass])
    /// - 'driver_type' - тип привода механизма подъема (enum [DriverType])
    /// - 'load_comb' - тип комбинации нагрузок (enum [LoadCombination])
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    pub fn eval(&mut self, storage: &mut Storage, user_hook: &UserHook,m_to_lift:f64, lift_class: LiftClass, driver_type: DriverType, load_comb: LoadCombination, vhmax: f64, vhcs: f64){
        let mut binding = BearingFilter::new();
        let mut filtered_bearings = binding.filter(storage, user_hook, m_to_lift, lift_class, driver_type, load_comb, vhmax, vhcs);
        if filtered_bearings.is_empty(){
            log::debug!("{}.eval | There is no bearing for your hook",self.dbgid);
        } else {
            for hook in filtered_bearings.iter(){
                log::debug!("{}", format!("{}.eval | {:?}", self.dbgid,hook.paint()));
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