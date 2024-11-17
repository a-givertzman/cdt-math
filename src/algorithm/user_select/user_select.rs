use std::str::FromStr;

use crate::{algorithm::storage::storage::Storage, kernel::entities::{crane_work_area::CraneWorkArea, driver_type::DriverType, hoist_rope_balance_degree::HoistRopeBalanceDegree, hoist_rope_twisting_method::HoistRopeTwistingMethod, hoist_rope_type::HoistRopeType, liftclass::LiftClass, load_combination::LoadCombination, mechanism_work_type::MechanismWorkType, value::Value}};

///
/// Класс, которой реализует хранение характеристики выбранные пользователем для дальнейшего расчета крана
pub struct UserSelect {
    pub dbgid: String,
    pub m_to_lift: f64,
    pub mechanism_work_type: MechanismWorkType,
    pub vhmax: f64,
    pub vhcs: f64,
    pub lift_class: LiftClass,
    pub load_comb: LoadCombination,
    pub drive_type: DriverType,
    pub hook_type: String,
    pub name_cargo_hand_device: String,
    pub weight_cargo_hand_device: f64,
    pub rejecting_blocks: f64,
    pub crane_work_area: CraneWorkArea,
    pub hoist_rope_core_type: HoistRopeType,
    pub hoist_rope_twisting_method: HoistRopeTwistingMethod,
    pub hoist_rope_balance_degree: HoistRopeBalanceDegree,
    pub hoist_rope_diametr: f64,

}
//
//
//
impl UserSelect {
    ///
    /// Метод создания экземпляра класса UserSelect
    /// - storage - экземпляр класса-хранилища Storage, в котором находится "таблица" конструкций, кранов, подшипников
    pub fn new(user_select_storage: &Storage) -> Self {
        let mut m_to_lift_tmp = 0.0;
        if let Some(value) = user_select_storage.get("конструкции/крюк/грузоподъемность/") {
            if let Value::Data(data) = value {
                m_to_lift_tmp = *data;
            }
        }

        let mut lift_class_tmp:LiftClass = LiftClass::Hc1;
        if let Some(value) = user_select_storage.get("конструкции/крюк/класс подъема/") {
            if let Value::String(data) = value {
                match LiftClass::from_str(&data){
                    Ok(liftclass) => lift_class_tmp = liftclass,
                    Err(_) => todo!(),
                }
            }
        };

        let mut load_comb_tmp:LoadCombination = LoadCombination::A1;
        if let Some(value) = user_select_storage.get("конструкции/крюк/комбинация нагрузок/") {
            if let Value::String(data) = value {
                match LoadCombination::from_str(&data){
                    Ok(load_comb) => load_comb_tmp = load_comb,
                    Err(_) => todo!(),
                }
            }
        };

        let mut drive_type_tmp:DriverType = DriverType::Hd1;
        if let Some(value) = user_select_storage.get("конструкции/крюк/тип привода/") {
            if let Value::String(data) = value {
                match DriverType::from_str(&data){
                    Ok(driver_type) => drive_type_tmp = driver_type,
                    Err(_) => todo!(),
                }
            }
        };

        let mut vhcs_tmp = 0.0;
        if let Some(value) = user_select_storage.get("конструкции/крюк/номинальная скорость подъема механизма/")
        {
            if let Value::Data(data) = value {
                vhcs_tmp = *data;
            }
        }

        let mut vhmax_tmp = 0.0;
        if let Some(value) = user_select_storage.get("конструкции/крюк/замедленная скорость подъема механизма/")
        {
            if let Value::Data(data) = value {
                vhmax_tmp = *data;
            }
        }

        let mut mechanism_work_type_tmp: MechanismWorkType = MechanismWorkType::M1;
        if let Some(value) = user_select_storage.get("конструкции/крюк/режим работы механизма/") {
            if let Value::String(data) = value {
                match MechanismWorkType::from_str(&data){
                    Ok(mechanism_work_type) => mechanism_work_type_tmp = mechanism_work_type,
                    Err(_) => todo!(),
                }
            }
        };

        let mut hook_type_tmp = String::new();
        if let Some(value) = user_select_storage.get("конструкции/крюк/тип крюка/") {
            if let Value::String(data) = value {
                hook_type_tmp = data.to_string();
            }
        }

        let mut name_cargo_tmp = String::new();
        if let Some(value) = user_select_storage.get("конструкции/крюк/тип грузозахватного органа механизма подъёма/")
        {
            if let Value::String(data) = value {
                name_cargo_tmp = data.to_string();
            }
        }

        let mut weight_cargo_tmp = 0.0;
        if let Some(value) =
            user_select_storage.get("конструкции/крюк/грузоподъемность грузозахватного органа механизма подъёма/")
        {
            if let Value::Data(data) = value {
                weight_cargo_tmp = *data;
            }
        }

        let mut rejecting_blocks_tmp = 0.0;
        if let Some(value) = user_select_storage.get("конструкции/канат/количество отклоняющих блоков для полиспаста/"){
            if let Value::Data(data) = value  {
                rejecting_blocks_tmp = *data;
            }
        }

        let mut crane_type_area_tmp: CraneWorkArea = CraneWorkArea::Normal;
        if let Some(value) = user_select_storage.get("конструкции/кран/тип рабочей среды крана/") {
            if let Value::String(data) = value {
                match CraneWorkArea::from_str(&data){
                    Ok(crane_type_area) => crane_type_area_tmp = crane_type_area,
                    Err(_) => todo!(),
                }
            }
        };

        let mut hoist_rope_core_type_tmp: HoistRopeType = HoistRopeType::Metal;
        if let Some(value) = user_select_storage.get("конструкции/канат/тип сердечника/") {
            if let Value::String(data) = value {
                match HoistRopeType::from_str(&data){
                    Ok(hoist_rope_core_type) => hoist_rope_core_type_tmp = hoist_rope_core_type,
                    Err(_) => todo!(),
                }
            }
        };

        let mut hoist_rope_twisting_method_tmp: HoistRopeTwistingMethod = HoistRopeTwistingMethod::Twisting;
        if let Some(value) = user_select_storage.get("конструкции/канат/способ свивки каната/") {
            if let Value::String(data) = value {
                match HoistRopeTwistingMethod::from_str(&data){
                    Ok(hoist_rope_twisting_method) => hoist_rope_twisting_method_tmp = hoist_rope_twisting_method,
                    Err(_) => todo!(),
                }
            }
        };

        let mut hoist_rope_balance_degree_tmp: HoistRopeBalanceDegree = HoistRopeBalanceDegree::Straightened;
        if let Some(value) = user_select_storage.get("конструкции/канат/степень уравновешенности каната/") {
            if let Value::String(data) = value {
                match HoistRopeBalanceDegree::from_str(&data){
                    Ok(hoist_rope_balance_degree) => hoist_rope_balance_degree_tmp = hoist_rope_balance_degree,
                    Err(_) => todo!(),
                }
            }
        };

        let mut hoist_rope_diametr_tmp = 0.0;
        if let Some(value) = user_select_storage.get("конструкции/канат/диаметр каната/"){
            if let Value::Data(data) = value  {
                hoist_rope_diametr_tmp = *data;
            }
        }

        Self {
            hoist_rope_twisting_method: hoist_rope_twisting_method_tmp,
            crane_work_area: crane_type_area_tmp,
            rejecting_blocks: rejecting_blocks_tmp,
            dbgid: String::from(format!("UserSelect")), 
            m_to_lift: m_to_lift_tmp,
            lift_class: lift_class_tmp,
            load_comb: load_comb_tmp,
            drive_type: drive_type_tmp,
            vhcs: vhcs_tmp,
            vhmax: vhmax_tmp,
            mechanism_work_type: mechanism_work_type_tmp,
            hook_type: hook_type_tmp,
            name_cargo_hand_device: name_cargo_tmp,
            weight_cargo_hand_device: weight_cargo_tmp,
            hoist_rope_core_type: hoist_rope_core_type_tmp,
            hoist_rope_balance_degree: hoist_rope_balance_degree_tmp,
            hoist_rope_diametr: hoist_rope_diametr_tmp,
        }
    }
}