use crate::kernel::dbgid::dbgid::DbgId;
///
/// Класс, рассчитывающий усилие в канате
/// [reference to effort in the rope documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct EffortInTheRope{
    dbgid: DbgId,
    value: f64
}
//
impl EffortInTheRope{
    ///
    /// Конструктор класса EffortInTheRope
    pub fn new() -> Self{
        Self { dbgid: DbgId(format!("EffortInTheRope")), value: 0.0 }
    }
    ///
    /// Метод, который выбирает усилие в канате относительно массе на крюковой подвеске
    /// [reference to effort in the rope documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    pub fn eval(&mut self, m_to_lift: f64) -> f64 {
        match m_to_lift {
            n if n <= 1.0 => self.value = 7.5,
            n if n <= 2.0 => self.value = 10.0,
            n if n <= 6.0 => self.value = 20.0,
            n if n <= 10.0 => self.value = 30.0,
            n if n <= 15.0 => self.value = 40.0,
            n if n <= 20.0 => self.value = 50.0,
            n if n <= 40.0 => self.value = 60.0,
            n if n <= 100.0 => self.value = 90.0,
            n if n <= 150.0 => self.value = 130.0,
            n if n <= 200.0 => self.value = 180.0,
            n if n <= 500.0 => self.value = 220.0,
            _ => {} // No action needed in the default case.
        }
        self.value
    }
}