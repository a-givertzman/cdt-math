use crate::kernel::crane_constructor::user::{self, user_select::UserSelect};

pub struct Cargo{
    pub cargo_name: String,
    pub cargo_weight: String,
    pub payload_weight: f64
}


impl Cargo {
    pub fn new(user_select: UserSelect) -> Self{

        let mut user_m = 0.0;
        match user_select.m_to_lift.trim().parse::<f64>() {
            Ok(value) => user_m = value,
            Err(e) => println!("error: {}", e),
        }

        let mut user_cw = 0.0;
        match user_select.cargo_weight.trim().parse::<f64>() {
            Ok(value) => user_m = value,
            Err(e) => println!("error: {}", e),
        }


        Self{
            cargo_name: user_select.cargo_name,
            cargo_weight: user_select.cargo_weight,
            payload_weight: Self::payload(user_m, user_cw)
        }
    }

    pub fn payload(load_capacity: f64, cargo_weight: f64) -> f64{
        load_capacity - cargo_weight
    }

}