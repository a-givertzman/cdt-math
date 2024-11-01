use crate::UserSelect;



// Готовые параметры для сравнения

struct Phi {
    min: f64,
    betta: f64,
}

pub struct Param_to_compare{
    pub _m_to_lift: f64,
    pub _m_work_type: String,
    pub _hook_type: String,
    pub _fmg : f64,
}

impl Param_to_compare{
    pub fn new(user_select: UserSelect) -> Self {
       let fmg = Self::get_fmg(user_select.m_to_lift,&user_select.lift_class,&user_select.load_comb,&user_select.drive_type,user_select.vhmax,user_select.vhcs);
        Self {
            _m_to_lift: user_select.m_to_lift,
            _m_work_type: user_select.m_work_type,
            _fmg: fmg, // Передача веса как параметра
            _hook_type: user_select.hook_type,
        }
    }
    
    pub fn bet_phi_chooser(lift_class: &str) -> (f64, f64) {
        match lift_class {
            "HC1" => (0.17, 1.05),
            "HC2" => (0.34, 1.10),
            "HC3" => (0.51, 1.15),
            "HC4" => (0.68, 1.20),
            _ => (0.0, 0.0),
        }
    }
    
    pub fn vh_chooser(load_comb: &str, drive_type: &str, vhmax: f64, vhcs: f64) -> f64 {
        match load_comb {
            "A1" | "B1" => match drive_type {
                "HD1" => vhmax,
                "HD2" | "HD3" => vhcs,
                "HD4" => vhmax * 0.5,
                "HD5" => 0.0,
                _ => 0.0,
            },
            "C1" => match drive_type {
                "HD1" | "HD2" | "HD4" => vhmax,
                "HD3" | "HD5" => vhmax * 0.5,
                _ => 0.0,
            },
            _ => 0.0,
        }
    }
    
    pub fn get_din_coeff((phi, bet): (f64,f64),vh: f64) -> f64 {
        phi + bet * vh
    }
    
    pub fn get_fmg(m_to_lift: f64,lift_class: &str,load_comb: &str, drive_type: &str, vhmax: f64, vhcs: f64) -> f64 {
        Self::get_din_coeff(Self::bet_phi_chooser(lift_class),Self::vh_chooser(load_comb, drive_type, vhmax, vhcs)) * m_to_lift * 9.81
    }
    

}