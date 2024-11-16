

#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct SummaryGoodWeights{
    pub(crate) summary_weight:f64,
    pub(crate) good_weight: f64,
}
//
//
//
impl SummaryGoodWeights{
    pub fn new() -> Self{
        Self { summary_weight: 0.0, good_weight: 0.0 }
    }

    pub fn eval(&mut self,hook_m: f64,weight_cargo_hand_device: f64, m_to_lift: f64){
        self.summary_weight = hook_m + weight_cargo_hand_device;
        self.good_weight = m_to_lift - weight_cargo_hand_device;
    }
}