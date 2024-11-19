use crate::{algorithm::{force_of_gravity::force_of_gravity::ForceOfGravity, hook_choose::user_hook::user_hook::UserHook, storage::storage::Storage}, kernel::{dbgid::dbgid::DbgId, entities::{bearing::Bearing, driver_type::DriverType, lift_class::LiftClass, load_combination::LoadCombination, value::Value}}};
///
/// Класс, реализующий фильтрацию подшипников
/// [reference to bearing filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'filtered_bearings' - вектор [Bearing] отфильтрованных подшипников
pub struct BearingFilter{
    dbgid: DbgId,
    filtered_bearings: Vec<Bearing>
}
//
impl BearingFilter{
    ///
    /// Конструктор класса BearingFilter
    pub fn new() -> Self{
        Self {  dbgid: DbgId(format!("BearingFilter")),
                filtered_bearings: Vec::new(),
        }
    }
    ///
    /// Метод фильтрации подшипников 
    /// [reference to bearing filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'storage' - БД [Storage]
    /// - 'user_hook' - крюк выбранный пользователем (экземпляр класса [UserHook])
    /// - 'm_to_lift' - масса на крюке
    /// - 'lift_class' - класс подъема (enum [LiftClass])
    /// - 'driver_type' - тип привода механизма подъема (enum [DriverType])
    /// - 'load_comb' - тип комбинации нагрузок (enum [LoadCombination])
    /// - 'vhmax' - номинальная скорость подъёма механизма
    /// - 'vhcs' - замедленная скорость подъёма механизма
    pub fn filter(&mut self, storage: &Storage,user_hook: &UserHook,m_to_lift:f64, lift_class: LiftClass, driver_type: DriverType, load_comb: LoadCombination, vhmax: f64, vhcs: f64) -> Vec<Bearing> {
        let fmg = match ForceOfGravity::new().eval(m_to_lift, lift_class, driver_type, load_comb, vhmax, vhcs){
            Ok(value) => value,
            Err(_) => 0.0,
        };
        let mut bearing_index= 0;
        loop{
            match storage.get_bearing(bearing_index){
                Some(bearing) => {
                    if fmg<= bearing.static_capacity{
                        if bearing.outer_diameter <= user_hook.user_hook.shank_diameter{
                            self.filtered_bearings.push(bearing.clone());
                        }
                    }
                    bearing_index+=1;
                },
                None => break,
            }
        }
        self.filtered_bearings.clone()
    }

}