use crate::kernel::crane_constructor::hook_chooser::middle_din_coeff::MiddDinCoeff;

pub struct DinCoeff{
    pub m_to_lift: f64,
    pub m_work_type: String,
    pub hook_type: String,
    pub result: f64
}

impl DinCoeff{
    pub fn new(mid_coeff: MiddDinCoeff) -> Self{

        let tmp_dc = Self::dincoeff(mid_coeff.phi2_min, mid_coeff.betta2, mid_coeff.vh);

        Self { m_to_lift: mid_coeff.m_to_lift, result: tmp_dc, m_work_type: mid_coeff.m_work_type, hook_type: mid_coeff.hook_type }
    }   

    pub fn dincoeff(phi2_min: f64, betta2: f64, vh: f64) -> f64{
        let res: f64 = phi2_min + betta2 * vh;
        res
    }

}