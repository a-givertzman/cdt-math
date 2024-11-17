use crate::{algorithm::cable_count::cable_count::CableCount, kernel::dbgid::dbgid::DbgId};

///
/// Класс, реализующий выбор типа полиспаста отностительно количества канатов
/// [reference to polispast type documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct PolispastType{
    dbgid: DbgId,
    pub(crate) cable_count: CableCount,
    value: u8
}
//
//
//
impl PolispastType{
    ///
    /// Конструктор класса PolispastType
    pub fn new() -> Self{
        Self { dbgid: DbgId(format!("PolispastType")), value: 0, cable_count: CableCount::new() }
    }
    ///
    /// Метод выбора типа полиспаста
    pub fn eval(&mut self,m_to_lift: f64, hook_weight: f64) -> u8{
        match self.cable_count.eval(m_to_lift, hook_weight){
            n if n == 2.0 => self.value = 1,
            n if n > 2.0 => self.value = 2,
            _ => {}
        }
        self.value
    }

}