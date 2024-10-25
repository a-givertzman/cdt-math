use crate::kernel::crane_constructor::hook_chooser::din_coeff::DinCoeff;

// Готовые параметры для сравнения
pub struct ParamComp{
    pub _m_to_lift: f64,
    pub _m_work_type: String,
    pub _hook_type: String,
    pub _fmg : f64
}

impl ParamComp{
    pub fn new(din_coeff: DinCoeff) -> Self{
        
        let tmp_fmg = Self::fmg(din_coeff.result, din_coeff.m_to_lift, 9.8); // g = 9.8 - скорость свободного падения

        Self {
            _m_to_lift: din_coeff.m_to_lift,
            _m_work_type: din_coeff.m_work_type,
            _fmg: tmp_fmg,
            _hook_type: din_coeff.hook_type
         }
    }

    pub fn fmg(din_coeff: f64, m_to_lift: f64, g: f64 ) -> f64{
        din_coeff*m_to_lift*g
    }

}