use crate::kernel::entities::mechanism_work_type::MechanismWorkType;

///
/// Класс, реализующий выбор коэффициента запаса каната 
/// [reference to maximum effort documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct RopeReserveRatio{
    value: f64
}
//
//
//
impl RopeReserveRatio{
    ///
    /// Конструктор класса
    pub fn new() -> Self{
        Self { value: 0.0 }
    }
    ///
    /// Метод выбора коэффициента запаса каната
    /// [reference to maximum effort documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub fn eval(&mut self,mechanism_work_type: MechanismWorkType) -> f64{
        match mechanism_work_type{
            MechanismWorkType::M1 => self.value = 3.15,
            MechanismWorkType::M2 => self.value = 3.35,
            MechanismWorkType::M3 => self.value = 3.55,
            MechanismWorkType::M4 => self.value = 4.0,
            MechanismWorkType::M5 => self.value = 4.5,
            MechanismWorkType::M6 => self.value = 5.6,
            MechanismWorkType::M7 => self.value = 7.1,
            MechanismWorkType::M8 => self.value = 9.0,
        }
        self.value
    }

}