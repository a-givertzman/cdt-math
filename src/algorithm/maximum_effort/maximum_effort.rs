use crate::{algorithm::{cable_count::cable_count::CableCount, efficiency_of_the_polyspast::efficiency_of_the_polyspast::EfficiencyOfThePolyspast}, kernel::dbgid::dbgid::DbgId};

///
/// Класс, для расчёта максимального усилия, возникающего в канате при подъёме номинального груза
/// - 'cable_count' - количество канатов, сходящих с крюковой подвески (экземпляр класса [CableCount])
/// - 'efficiency_of_the_polyspast' - коэффициент полезного действия полиспаста (экземпляр класса [EfficiencyOfThePolyspast])
pub struct MaximumEffort{
    dbgid: DbgId,
    cable_count: CableCount,
    efficiency_of_the_polyspast: EfficiencyOfThePolyspast,
    g: f64,
    value: f64,
}
//
//
//
impl MaximumEffort{
    ///
    /// Конструктор класса MaximumEffort
    pub fn new() ->Self{
        Self {  dbgid: DbgId(format!("MaximumEffort")),
                cable_count: CableCount::new(),
                efficiency_of_the_polyspast: EfficiencyOfThePolyspast::new(),
                g: 9.81,
                value: 0.0, 
             }
    }
    ///
    /// Метод расчёта максимального усилия, возникающего в канате при подъёме номинального груза
    pub fn eval(&mut self,m_to_lift: f64, hook_weight: f64, rejecting_blocks: f64) ->f64{
        self.value = (m_to_lift+hook_weight)*self.g/(self.cable_count.eval(m_to_lift, hook_weight)*self.efficiency_of_the_polyspast.eval(m_to_lift, hook_weight, rejecting_blocks));
        self.value
    }
}