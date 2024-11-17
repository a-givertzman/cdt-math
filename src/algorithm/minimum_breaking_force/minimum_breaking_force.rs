use crate::{algorithm::{maximum_effort::maximum_effort::MaximumEffort, rope_reserve_ratio::rope_reserve_ratio::RopeReserveRatio}, kernel::entities::mechanism_work_type::MechanismWorkType};

///
/// Класс реализующий расчёт необходимого минимального разрывного усилия
/// [reference to minimum breaking force  documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
/// - 'maximum_effort' - максимальное усилие, возникающее в канате при подъёме номинального груза (экземпляр класса [MaximumEffort])
/// - 'rope_reserve_ratio' - коэффициент запаса каната (экземпляр класса [RopeReserveRatio])
pub struct MinimumBreakingForce{
    maximum_effort: MaximumEffort,
    rope_reserve_ratio: RopeReserveRatio,
    value: f64,
}
//
//
//
impl MinimumBreakingForce{
    ///
    /// Конструктор класса MinimumBreakingForce
    pub fn new() -> Self{
        Self {  maximum_effort: MaximumEffort::new(),
                rope_reserve_ratio: RopeReserveRatio::new(),
                value: 0.0, 
             }
    }
    ///
    /// Метод расчёта необходимого минимального разрывного усилия
    /// [reference to minimum breaking force  documentation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    /// - 'm_to_lift' - масса на крюке
    /// - 'hook_weight' - масса крюковой подвески
    /// - 'rejecting_blocks' - количество отклоняющих блоков
    /// - 'mechanism_work_type' - режим работы механизма подъема (enum [MechanismWorkType])
    pub fn eval(&mut self,m_to_lift: f64, hook_weight: f64, rejecting_blocks: f64,mechanism_work_type: MechanismWorkType) ->f64{
        self.value = self.maximum_effort.eval(m_to_lift, hook_weight, rejecting_blocks)*self.rope_reserve_ratio.eval(mechanism_work_type);
        self.value
    }
}