use std::io;
use std::string;
pub struct UserSelect{
    //все свойства для выбора крана ( пока только для крюка)
    pub m_to_lift: String,
    // для динамического коэффициента
    pub m_work_type: String,
    pub vhmax: String,
    pub vhcs: String,
    pub lift_class: String,
    pub load_comb: String,
    pub drive_type: String, 
    pub hook_type: String,
    pub cargo_name: String,
    pub cargo_weight: String
}

impl UserSelect {
    pub fn new() -> Self{
            println!("Enter characteristics to select right hooks!");
            let mut m_str = String::new();

            let mut hook_type_str = String::new();

            let mut m_work_type_str = String::new();

            let mut lift_class_str = String::new();

            let mut load_comb_str = String::new();

            let mut drive_type_str = String::new();

            let mut vhcs_str = String::new();

            let mut vhmax_str = String::new();
            println!("Enter load capacity:");
            match io::stdin().read_line(&mut m_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            println!("Enter the lifting class:");
            match io::stdin().read_line(&mut lift_class_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            println!("Enter the load combination:");
            match io::stdin().read_line(&mut load_comb_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            println!("Enter the type of drive:");
            match io::stdin().read_line(&mut drive_type_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            println!("Enter the slow lifting speed of the mechanism:");
            match io::stdin().read_line(&mut vhcs_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            println!("Enter the nominal lifting speed of the mechanism:");
            match io::stdin().read_line(&mut vhmax_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            
            println!("Enter the operating mode of the mechanism according to GOST 34017-2016:");
            match io::stdin().read_line(&mut m_work_type_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }
            
            println!("Enter the hook type:");
            match io::stdin().read_line(&mut hook_type_str) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }

            println!("Enter name of cargo!");
            let mut name_cargo = String::new();
            match io::stdin().read_line(&mut name_cargo) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }

            println!("Enter weight of cargo!");
            let mut weight_cargo = String::new();
            match io::stdin().read_line(&mut weight_cargo) {
                Ok(_) => {},
                Err(e) => println!("Input error! {}",e)
            }


            Self { m_to_lift: m_str, 
                lift_class: lift_class_str, 
                load_comb: load_comb_str, 
                drive_type: drive_type_str, 
                vhcs: vhcs_str, 
                vhmax: vhmax_str,
                m_work_type: m_work_type_str,
                hook_type: hook_type_str,
                cargo_name: name_cargo,
                cargo_weight: weight_cargo
            }
    }
}