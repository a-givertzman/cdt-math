use std::str::FromStr;

use crate::{algorithm::storage::storage::Storage, kernel::entities::{driver_type::DriverType, liftclass::LiftClass, load_combination::LoadCombination, value::Value}};

///
/// Класс, которой реализует хранение характеристики выбранные пользователем для дальнейшего расчета крана
pub struct UserSelect {
    pub dbgid: String,
    pub m_to_lift: f64,
    pub mechanism_work_type: String,
    pub vhmax: f64,
    pub vhcs: f64,
    pub lift_class: LiftClass,
    pub load_comb: LoadCombination,
    pub drive_type: DriverType,
    pub hook_type: String,
    pub cargo_name: String,
    pub cargo_weight: f64,
}
//
//
//
impl UserSelect {
    ///
    /// Метод создания экземпляра класса UserSelect
    /// - storage - экземпляр класса-хранилища Storage, в котором находится "таблица" конструкций, кранов, подшипников
    /// 
    pub fn new(storage: &Storage) -> Self {
        let mut m_to_lift_tmp = 0.0;
        if let Some(value) = storage.get("грузоподъемность/") {
            if let Value::Data(data) = value {
                m_to_lift_tmp = *data;
            }
        }

        let mut lift_class_tmp:LiftClass = LiftClass::Hc1;
        if let Some(value) = storage.get("класс подъема/") {
            if let Value::String(data) = value {
                match LiftClass::from_str(&data){
                    Ok(liftclass) => lift_class_tmp = liftclass,
                    Err(_) => todo!(),
                }
            }
        };

        let mut load_comb_tmp:LoadCombination = LoadCombination::A1;
        if let Some(value) = storage.get("комбинация нагрузок/") {
            if let Value::String(data) = value {
                match LoadCombination::from_str(&data){
                    Ok(load_comb) => load_comb_tmp = load_comb,
                    Err(_) => todo!(),
                }
            }
        };

        let mut drive_type_tmp:DriverType = DriverType::Hd1;
        if let Some(value) = storage.get("тип привода/") {
            if let Value::String(data) = value {
                match DriverType::from_str(&data){
                    Ok(driver_type) => drive_type_tmp = driver_type,
                    Err(_) => todo!(),
                }
            }
        };

        let mut vhcs_tmp = 0.0;
        if let Some(value) = storage.get("номинальная скорость подъема механизма/")
        {
            if let Value::Data(data) = value {
                vhcs_tmp = *data;
            }
        }

        let mut vhmax_tmp = 0.0;
        if let Some(value) = storage.get("замедленная скорость подъема механизма/")
        {
            if let Value::Data(data) = value {
                vhmax_tmp = *data;
            }
        }

        let mut m_work_type_tmp = String::new();
        if let Some(value) = storage.get("режим работы механизма/") {
            if let Value::String(data) = value {
                m_work_type_tmp = data.to_string();
            }
        }

        let mut hook_type_tmp = String::new();
        if let Some(value) = storage.get("тип крюка/") {
            if let Value::String(data) = value {
                hook_type_tmp = data.to_string();
            }
        }

        let mut name_cargo_tmp = String::new();
        if let Some(value) = storage.get("тип грузозахватного органа механизма подъёма/")
        {
            if let Value::String(data) = value {
                name_cargo_tmp = data.to_string();
            }
        }

        let mut weight_cargo_tmp = 0.0;
        if let Some(value) =
            storage.get("грузоподъемность грузозахватного органа механизма подъёма/")
        {
            if let Value::Data(data) = value {
                weight_cargo_tmp = *data;
            }
        }

        Self {
            dbgid: String::from(format!("UserSelect")), 
            m_to_lift: m_to_lift_tmp,
            lift_class: lift_class_tmp,
            load_comb: load_comb_tmp,
            drive_type: drive_type_tmp,
            vhcs: vhcs_tmp,
            vhmax: vhmax_tmp,
            mechanism_work_type: m_work_type_tmp,
            hook_type: hook_type_tmp,
            cargo_name: name_cargo_tmp,
            cargo_weight: weight_cargo_tmp,
        }
    }
}