use crate::kernel::crane_constructor::user::{self, user_select::UserSelect};
use crate::kernel::crane_constructor::hook_chooser::hook::Hook;

pub struct HoistingTackle {
    pub hoisting_tackle: i8,
    pub cable_count: f64,
    pub multiplicity_of_polispast: f64,
    hook_summary_weight: f64,
    m_to_lift: f64,
}

impl HoistingTackle {
    pub fn new(hook: &Hook) -> Self {
        Self {
            m_to_lift: hook.m_to_lift,
            hook_summary_weight: hook.summary_weight,
            cable_count: 0.0,
            hoisting_tackle: 0,
            multiplicity_of_polispast: 0.0
        }
    }

    pub fn eval(&mut self){
        let s = Self::s_select(self.m_to_lift);
        let tmp_cable_count = Self::cable_count(s, self.m_to_lift, self.hook_summary_weight);
        let tmp_hoisting_tackle = Self::a_select(tmp_cable_count);
        let multi = tmp_cable_count/(tmp_hoisting_tackle as f64);

        self.cable_count = tmp_cable_count;
        self.hoisting_tackle = tmp_hoisting_tackle;
        self.multiplicity_of_polispast = multi;
    }

    pub fn a_select(n: f64) -> i8 {
        if n > 2.0 { 2 } else { 1 }
    }

    pub fn cable_count(s: f64, m_to_lift: f64, w_hook: f64) -> f64 {

        Self::round_to_nearest((m_to_lift + w_hook) / s)
    }

    pub fn round_to_nearest(x: f64) -> f64 {
        let options = [2.0, 4.0, 8.0, 12.0, 16.0];
        
        options
            .iter()
            .filter(|&&val| val >= x)  // Keep only options >= x
            .min_by(|a, b| (x - **a).abs().partial_cmp(&(x - **b).abs()).unwrap())
            .cloned()  // Convert from &f64 to f64
            .unwrap_or_else(|| *options.last().unwrap())  // Default to the highest option if all are less than x
    }
    

    pub fn s_select(m: f64) -> f64 {
        match m {
            n if n <= 1.0 => 7.5,
            n if n <= 2.0 => 10.0,
            n if n <= 6.0 => 20.0,
            n if n <= 10.0 => 30.0,
            n if n <= 15.0 => 40.0,
            n if n <= 20.0 => 50.0,
            n if n <= 40.0 => 60.0,
            n if n <= 100.0 => 90.0,
            n if n <= 150.0 => 130.0,
            n if n <= 200.0 => 180.0,
            n if n <= 500.0 => 220.0,
            _ => {
                println!("Значение вне диапазона");
                0.0
            }
        }
    }
}
