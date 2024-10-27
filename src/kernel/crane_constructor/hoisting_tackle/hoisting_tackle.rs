use crate::kernel::crane_constructor::user::{self, user_select::UserSelect};
use crate::kernel::crane_constructor::hook_chooser::hook::Hook;

pub struct hoisting_tackle{
    pub hoisting_tackle: i8,
    pub cable_count: f64
}

impl hoisting_tackle{
    pub fn new(user: &UserSelect,hook: &Hook) -> Self{
        
        let mut n  = Self::cable_count(Self::s_select(&user.m_to_lift), &user.m_to_lift, &hook.hook[4]);
        let mut a = Self::a_select(n);

        Self{
            cable_count: n,
            hoisting_tackle: a
        }


    }

    fn a_select(n: f64) -> i8{
        if n > 2.0{
            2
        }
        else {
            1
        }
    }

    pub fn cable_count(s: f64, m_to_lift: &String, w_hook: &String) -> f64{
        let mut M: f64 = 0.0;
        match m_to_lift.trim().parse::<f64>() {
            Ok(value) => M = value,
            Err(e) => { println!("error: {}", e);  return 0.0 },
        }

        let mut m: f64 = 0.0;
        match w_hook.trim().parse::<f64>() {
            Ok(value) => m = value,
            Err(e) => { println!("error: {}", e); return 0.0 },
        }


        Self::round_to_nearest((M+m)/s)

    } 

    fn round_to_nearest(x:f64) -> f64 {
        let options = [2.0, 4.0, 8.0, 12.0, 16.0];
        *options
            .iter()
            .min_by(|a, b| ((x - **a).abs()).partial_cmp(&(x - **b).abs()).unwrap())
            .unwrap()
    }


    fn s_select(M: &String) -> f64{
        let mut user_m = 0.0;
        match M.trim().parse::<f64>() {
            Ok(value) => user_m = value,
            Err(e) => println!("error: {}", e),
        }

        let s = match user_m {
            n if n <= 1. => 7.5,
            n if n <= 2. => 10.0,
            n if n <= 6. => 20.0,
            n if n <= 10. => 30.0,
            n if n <= 15. => 40.0,
            n if n <= 20. => 50.0,
            n if n <= 40. => 60.0,
            n if n <= 100. => 90.0,
            n if n <= 150. => 130.0,
            n if n <= 200. => 180.0,
            n if n <= 500. => 220.0,
            _ => {
                println!("Значение вне диапазона");
                0.0
            }
        };

        s

    }

}