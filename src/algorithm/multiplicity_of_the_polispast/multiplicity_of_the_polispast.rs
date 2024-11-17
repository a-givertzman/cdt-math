use crate::algorithm::polispast_type::polispast_type::PolispastType;

///
/// Класс, который рассчитывает кратность полиспаста
/// [reference to the multiplicity of the polispast documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct MultiplicityOfThePolispast{
    polispast_type: PolispastType,
    value: f64
}
//
//
//
impl MultiplicityOfThePolispast{
    ///
    /// Конструктор класса MultiplicityOfThePolispast
    pub fn new() -> Self{
        Self { polispast_type: PolispastType::new(), value: 0.0 }
    }
    ///
    /// Метод расчёта кратности полиспаста
    /// [reference to the multiplicity of the polispast documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    pub fn eval(&mut self,m_to_lift:f64, hook_weight: f64) -> f64{
        let value = self.polispast_type.eval(m_to_lift, hook_weight);
        self.value = self.polispast_type.cable_count.value/(value as f64);
        self.value
    }
}