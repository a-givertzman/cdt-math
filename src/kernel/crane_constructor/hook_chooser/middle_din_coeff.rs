use crate::kernel::crane_constructor::user::user_select::UserSelect;
pub struct MiddDinCoeff{
    pub m_to_lift: f64,
    pub hook_type: String,
    pub vh: f64,
    pub m_work_type: String,
    pub betta2: f64,
    pub phi2_min: f64,
}

impl MiddDinCoeff{

    
    pub fn new(user_select: &UserSelect) -> Self{
        //println!("{}",user_select.m_to_lift);
        let mut user_m = 0.0;
        match user_select.m_to_lift.trim().parse::<f64>() {
            Ok(value) => user_m = value,
            Err(e) => println!("error: {}", e),
        }


        let tmp_vh = Self::vh_chooser(user_select.load_comb.as_str(),user_select.drive_type.as_str(),&user_select.vhmax,&user_select.vhcs);
        
        
        let (tmp_betta2, tmp_phi2_min) = Self::bet_phi_chooser(user_select.lift_class.as_str());

        
        Self {
            m_to_lift: user_m,
            vh: tmp_vh,
            betta2: tmp_betta2,
            phi2_min: tmp_phi2_min,
            m_work_type: user_select.m_work_type.clone(),
            hook_type: user_select.hook_type.clone()
        }

    }


    pub fn bet_phi_chooser(lift_class: &str) -> (f64,f64){
        match lift_class{
            "HC1" => { (0.17, 1.05) },
            "HC2" => { (0.34, 1.10) },
            "НС3" => { (0.51, 1.15) },
            "HC4" => { (0.68, 1.20) },
            _ => { (0.0, 0.0) }
        }
    }


    pub fn vh_chooser(load_comb: &str, drive_type: &str,user_vhmax: &String, user_vhcs: &String ) -> f64{
        let mut vhmax = 0.0;
        match user_vhmax.trim().parse::<f64>() {
            Ok(number) => vhmax = number,
            Err(e) => println!("error: {}", e),
        }

        let mut vhcs = 0.0;
        match user_vhcs.trim().parse::<f64>() {
            Ok(number) => vhcs = number,
            Err(e) => println!("error: {}", e),
        }

        match load_comb{
            "A1" =>
                    match drive_type{
                        "HD1" => vhmax, // vhmax
                        "HD2" => vhcs, // vhcs
                        "HD3" => vhcs, // vhcs
                        "HD4" => vhmax * 0.5, // 0.5 vhmax
                        "HD5" => 0.0, // vh = 0
                        _ => 0.0
                    }

            "C1" =>
                    match drive_type{
                        "HD1" => vhmax, // vhmax
                        "HD2" => vhmax, // vhmax
                        "HD3" => vhmax * 0.5, // 0.5 vhmax
                        "HD4" => vhmax, // vhmax
                        "HD5" => vhmax * 0.5, // 0.5 vhmax
                        _ => 0.0
                    }
            
            _ => 0.0
        }
    }

    
}