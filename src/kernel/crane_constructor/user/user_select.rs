use std::io;
use std::string;
use crate::Storage;
use crate::kernel::storage::storage::Value;
///
/// Класс, которой реализует характеристики выбранные пользователем для дальнейшего расчета крана
/// 
pub struct UserSelect{
    pub dbgid: String,
    pub m_to_lift: f64,
    pub m_work_type: String,
    pub vhmax: f64,
    pub vhcs: f64,
    pub lift_class: String,
    pub load_comb: String,
    pub drive_type: String, 
    pub hook_type: String,
    pub cargo_name: String,
    pub cargo_weight: f64,
    pub crane_work_type: f64,
    pub rope_type: String,
    pub crane_type_area: String,
    pub rejecting_blocks: f64,
    pub rope_count: f64,
    pub rope_diametr: f64,
    pub rope_twisting_method: String,
    pub balance_degree: String,
}
//
//
//
impl UserSelect {
    ///
    /// Метод создания экземпляра класса UserSelect
    /// - storage - экземпляр класса-хранилища Storage, в котором находится "таблица" конструкций, кранов, подшипников
    /// 
    pub fn new(storage: Storage) -> Self{            
            let mut m_to_lift_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/крюк/грузоподъемность/"){
                if let Value::Data(data) = value  {
                    m_to_lift_tmp = *data;
                }
            }

            let mut lift_class_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/класс подъема/"){
                if let Value::String(data) = value  {
                    lift_class_tmp = data.to_string();
                }
            }
            
            let mut load_comb_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/комбинация нагрузок/"){
                if let Value::String(data) = value  {
                    load_comb_tmp = data.to_string();
                }
            }

            let mut drive_type_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/тип привода/"){
                if let Value::String(data) = value  {
                    drive_type_tmp = data.to_string();
                }
            }

            let mut vhcs_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/крюк/номинальная скорость подъема механизма/"){
                if let Value::Data(data) = value  {
                    vhcs_tmp = *data;
                }
            }


            let mut vhmax_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/крюк/замедленная скорость подъема механизма/"){
                if let Value::Data(data) = value  {
                    vhmax_tmp = *data;
                }
            }

            let mut m_work_type_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/режим работы механизма/"){
                if let Value::String(data) = value  {
                    m_work_type_tmp = data.to_string();
                }
            }

            let mut hook_type_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/тип крюка/"){
                if let Value::String(data) = value  {
                    hook_type_tmp = data.to_string();
                }
            }

            let mut name_cargo_tmp = String::new();
            if let Some(value) = storage.get("конструкции/крюк/тип грузозахватного органа механизма подъёма/"){
                if let Value::String(data) = value  {
                    name_cargo_tmp = data.to_string();
                }
            }

            let mut weight_cargo_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/крюк/грузоподъемность грузозахватного органа механизма подъёма/"){
                if let Value::Data(data) = value  {
                    weight_cargo_tmp = *data;
                }
            }

            let mut crane_work_type_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/крюк/грузоподъемность грузозахватного органа механизма подъёма/"){
                if let Value::Data(data) = value  {
                    crane_work_type_tmp = *data;
                }
            }

            let mut rope_type_tmp = String::new();
            if let Some(value) = storage.get("конструкции/канат/тип сердечника/"){
                if let Value::String(data) = value  {
                    rope_type_tmp = data.to_string();
                }
            }

            let mut crane_type_area_tmp = String::new();
            if let Some(value) = storage.get("конструкции/кран/ветровой район расположения крана/"){
                if let Value::String(data) = value  {
                    crane_type_area_tmp = data.to_string();
                }
            }

            let mut rejecting_blocks_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/канат/количество отклоняющих блоков для полиспаста/"){
                if let Value::Data(data) = value  {
                    rejecting_blocks_tmp = *data;
                }
            }

            let mut rope_count_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/канат/количество канатов маркировочной группы/"){
                if let Value::Data(data) = value  {
                    rope_count_tmp = *data;
                }
            }

            let mut rope_diametr_tmp = 0.0;
            if let Some(value) = storage.get("конструкции/канат/диаметр каната/"){
                if let Value::Data(data) = value  {
                    rope_diametr_tmp = *data;
                }
            }

            let mut twisting_method_tmp = String::new();
            if let Some(value) = storage.get("конструкции/канат/способ свивки каната/"){
                if let Value::String(data) = value  {
                    twisting_method_tmp = data.to_string();
                }
            }

            let mut balance_degree_tmp = String::new();
            if let Some(value) = storage.get("конструкции/канат/степень уравновешенности каната/"){
                if let Value::String(data) = value  {
                    balance_degree_tmp = data.to_string();
                }
            }

            Self {
                balance_degree: balance_degree_tmp,
                rope_twisting_method: twisting_method_tmp,
                rope_diametr: rope_diametr_tmp,
                rope_count: rope_count_tmp,
                rejecting_blocks: rejecting_blocks_tmp,
                crane_type_area: crane_type_area_tmp,
                rope_type: rope_type_tmp,
                crane_work_type: crane_work_type_tmp,
                dbgid: String::from(format!("{}/UserSelect",storage.dbgid)), 
                m_to_lift: m_to_lift_tmp, 
                lift_class: lift_class_tmp, 
                load_comb: load_comb_tmp, 
                drive_type: drive_type_tmp, 
                vhcs: vhcs_tmp, 
                vhmax: vhmax_tmp,
                m_work_type: m_work_type_tmp,
                hook_type: hook_type_tmp,
                cargo_name: name_cargo_tmp,
                cargo_weight: weight_cargo_tmp
            }
    }
}