use crate::kernel::{dbgid::dbgid::DbgId, entities::hoist_rope_type::HoistRopeType};
///
/// Класс, содержащий информацию о канате
/// [reference to hoist rope information documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
/// - 'standart' - стандарт каната                                      
/// - 'hoist_rope_diametr' - диаметр каната
/// - 'hoist_rope_core_type' - тип каната
/// - 'rope_strength_class' - класс прочности каната
/// - 'breaking_force_of_the_rope' - разрывное усилие каната
/// - 'the_cross_sectional_area_of_the_rope' - площадь сечения каната
/// - 'specific_gravity_of_the_rope' - удельная масса каната
#[derive(Debug, Clone)]
pub struct HoistRope{
    dbgid: DbgId,
    standart: String,
    hoist_rope_diametr: f64,
    hoist_rope_core_type: HoistRopeType,
    rope_strength_class: String,
    breaking_force_of_the_rope: f64,
    the_cross_sectional_area_of_the_rope: f64,
    specific_gravity_of_the_rope: f64,
}
//
//
//
impl HoistRope{
    ///
    /// Конструктор класса HoistRope
    /// [reference to hoist rope information documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    /// - 'standart' - стандарт каната                                      
    /// - 'hoist_rope_diametr' - диаметр каната
    /// - 'hoist_rope_core_type' - тип каната
    /// - 'rope_strength_class' - класс прочности каната
    /// - 'breaking_force_of_the_rope' - разрывное усилие каната
    /// - 'the_cross_sectional_area_of_the_rope' - площадь сечения каната
    /// - 'specific_gravity_of_the_rope' - удельная масса каната
    pub fn new(standart: String,
        hoist_rope_diametr: f64,
        hoist_rope_core_type: HoistRopeType,
        rope_strength_class: String,
        breaking_force_of_the_rope: f64,
        the_cross_sectional_area_of_the_rope: f64,
        specific_gravity_of_the_rope: f64) -> Self{
            Self { dbgid: DbgId(format!("HoistRope")), standart, hoist_rope_diametr, hoist_rope_core_type, rope_strength_class, breaking_force_of_the_rope, the_cross_sectional_area_of_the_rope, specific_gravity_of_the_rope }
    }
    ///
    /// Метод вывода информации о канате в консоль
    pub fn paint(&self){
        log::debug!("{}", format!("{}.print | Hoist rope standart: {}",self.dbgid,self.standart));
        log::debug!("{}", format!("{}.print | Hoist rope diametr: {}",self.dbgid,self.hoist_rope_diametr));
        log::debug!("{}", format!("{}.print | Hoist rope core type: {}",self.dbgid,self.hoist_rope_core_type.to_string()));
        log::debug!("{}", format!("{}.print | Hoist rope strength class: {}",self.dbgid,self.rope_strength_class));
        log::debug!("{}", format!("{}.print | Hoist rope breaking force: {}",self.dbgid,self.breaking_force_of_the_rope));
        log::debug!("{}", format!("{}.print | Hoist rope cross sectional area: {}",self.dbgid,self.the_cross_sectional_area_of_the_rope));
        log::debug!("{}", format!("{}.print | Hoist rope cross specific_gravity: {}",self.dbgid,self.specific_gravity_of_the_rope));
    }
}