use crate::algorithm::cable_count::cable_count::CableCount;

///
/// Класс, рассчитывающий натяжение в канате
/// [reference to tension in the rope documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
/// - 'cable_count' - количество канатов, сходящих с крюковой подвески (экземпляр класса [CableCount])
pub struct TensionInTheRope{
    cable_count: CableCount,
    value: f64,
}
//
//
//
impl TensionInTheRope{
    ///
    /// Конструктор класса TensionInTheRope
    pub fn new() -> Self{
        Self { cable_count: CableCount::new(), value: 0.0 }
    }
    ///
    /// Метод расчёта натяжение в канате
    /// - 'm_to_lift' - масса на крюке
    /// - 'hook_weight' - масса крюковой подвески
    pub fn eval(&mut self,m_to_lift: f64,hook_weight: f64) -> f64{
        self.value = m_to_lift/self.cable_count.eval(m_to_lift, hook_weight);
        self.value
    }
}