use crate::kernel::crane_constructor::user::{self, user_select::UserSelect};

use super::hook::Hook;

pub struct Cargo{
    pub cargo_name: String,
    pub cargo_weight_summary: f64,
    pub payload_weight: f64
}


impl Cargo {
    pub fn new(user_select: &UserSelect,hook: &Hook) -> Self{

        let user_m = user_select.m_to_lift;

        let user_cw = user_select.cargo_weight;

        let mut hook_w = 0.0;
        match hook.hook[4].clone().trim().parse::<f64>(){
            Ok(value) => hook_w = value,
            Err(e) => println!("error: {}", e),
        }

        Self{
            cargo_name: user_select.cargo_name.clone(),
            cargo_weight_summary: Self::summary_weight(hook_w, user_cw),
            payload_weight: Self::payload(user_m, user_cw)
        }
    }

    fn summary_weight(m_hook: f64, m_cargo: f64) -> f64{
        m_hook + m_cargo
    }

    fn payload(load_capacity: f64, cargo_weight: f64) -> f64{
        load_capacity - cargo_weight
    }

}