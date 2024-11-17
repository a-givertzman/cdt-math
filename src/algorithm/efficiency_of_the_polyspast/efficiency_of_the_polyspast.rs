use crate::{algorithm::{multiplicity_of_the_polispast::multiplicity_of_the_polispast::MultiplicityOfThePolispast, user_select::user_select::UserSelect}, kernel::entities::coeff_deflecting_and_bypass::CoeffDeflectingAndBypass};

///
/// Класс, реализующий расчёт коэффициент полезного действия полиспаста
/// - 'coeff_defl_bypass' - коэффициенты  полезного действия отклоняющих/обводных  канатных блоков (экземпляр класса [CoeffDeflectingAndBypass])
/// - 'rejecting_blocks' - количество отклоняющих блоков для полиспаста
/// - 'multiplicity_of_the_polispast' - кратность полиспаста (экземпляр класса [MultiplicityOfThePolispast])
pub struct EfficiencyOfThePolyspast{
    coeff_defl_bypass: CoeffDeflectingAndBypass,
    multiplicity_of_the_polispast: MultiplicityOfThePolispast,
    value: f64,
}
//
//
//
impl EfficiencyOfThePolyspast {
    ///
    /// Конструктор класса EfficiencyOfThePolyspast
    pub fn new() -> Self{
        Self {  coeff_defl_bypass: CoeffDeflectingAndBypass::new(),
                multiplicity_of_the_polispast: MultiplicityOfThePolispast::new(),
                value: 0.0,
             }
    }
    ///
    /// Метод расчёта полезного действия полиспаста
    /// - 'm_to_lift' - масса на крюке
    /// - 'hook_weight' - масса крюковой подвески
    pub fn eval(&mut self, m_to_lift: f64, hook_weight: f64, rejecting_blocks: f64) -> f64{
        self.value = f64::powf(self.coeff_defl_bypass.deflecting_coeff,rejecting_blocks)*(1.0-f64::powf(self.coeff_defl_bypass.bypass_coeff,self.multiplicity_of_the_polispast.eval(m_to_lift, hook_weight)))/
        ((1.0-self.coeff_defl_bypass.bypass_coeff)*self.multiplicity_of_the_polispast.eval(m_to_lift, hook_weight));
        self.value
    }
}
