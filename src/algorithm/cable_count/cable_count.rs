use crate::{algorithm::effort_in_the_rope::effort_in_the_rope::EffortInTheRope, kernel::dbgid::dbgid::DbgId};
///
/// Класс, который рассчитывает количество канатов, сходящих с крюковой подвески
/// - 'effort_in_the_rope' - усилие в канате экземляр класса [EffortInTheRope]
/// [reference to cable count documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct CableCount{
    dbgid: DbgId,
    effort_in_the_rope: EffortInTheRope,
    value: f64,
}
//
impl CableCount{
    ///
    /// Экземпляр класса CableCount
    pub fn new() -> Self{
        Self { dbgid: DbgId(format!("CableCount"))
        , value: 0.0,
        effort_in_the_rope: EffortInTheRope::new() 
        }
    }
    ///
    /// Метод рассчитывающий количество канатов, сходящих с крюковой подвески
    /// - 'm_to_lift' - масса на крюке
    /// - 'hook_weight' - масса крюковой подвески
    /// [reference to cable count documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    pub fn eval(&mut self,m_to_lift: f64, hook_weight: f64) -> f64{
        self.value = Self::round_to_nearest((m_to_lift+hook_weight)/self.effort_in_the_rope.eval(m_to_lift));
        self.value
    }
    ///
    /// Метод округления до ближайщего из списка
    /// - 'x' - число для округления 
    /// [reference to rounding documentation](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    pub fn round_to_nearest(x: f64) -> f64 {
        let options = [2.0, 4.0, 8.0, 12.0, 16.0];
        
        options
            .iter()
            .filter(|&&val| val >= x)  // Keep only options >= x
            .min_by(|a, b| (x - **a).abs().partial_cmp(&(x - **b).abs()).unwrap())
            .cloned()  // Convert from &f64 to f64
            .unwrap_or_else(|| *options.last().unwrap())  // Default to the highest option if all are less than x
    }
}

