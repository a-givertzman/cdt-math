use serde::{Deserialize, Serialize};
use crate::kernel::{dbgid::dbgid::DbgId, entities::{crane_work_area::CraneWorkArea, driver_type::DriverType, hoist_rope_balance_degree::HoistRopeBalanceDegree, hoist_rope_twisting_method::HoistRopeTwistingMethod, lift_class::LiftClass, load_combination::LoadCombination, mechanism_work_type::MechanismWorkType}};
///
/// Класс, которой реализует хранение характеристики выбранные пользователем для дальнейшего расчета крана
/// - 'm_to_lift' - масса на крюке
/// - 'lift_class' - класс подъема
/// - 'load_combination' - тип комбинации нагрузок
/// - 'driver_type' - тип привода механизма подъема
/// - 'vhmax' - номинальная скорость подъёма механизма
/// - 'vhcs' - замедленная скорость подъёма механизма
/// - 'lifting_mechanism_work_type' - тип работы механизма подъема
/// - 'hook_type' - тип крюка
/// - 'name_cargo_hand_device' - имя допольнительного грузозахватного органа
/// - 'weight_cargo_hand_device' - масса допольнительного грузозахватного органа
/// [reference to initial data documentation](design\docs\algorithm\part01\initial_data.md)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSelect {
    pub dbgid: DbgId,
    pub m_to_lift: f64,
    pub lift_class: LiftClass,
    pub load_combination: LoadCombination,
    pub driver_type: DriverType,
    pub vhmax: f64,
    pub vhcs: f64,
    pub lifting_mechanism_work_type: MechanismWorkType,
    pub hook_type: String,
    pub name_cargo_hand_device: String,
    pub weight_cargo_hand_device: f64,
    pub crane_work_area: CraneWorkArea,
    pub hoist_rope_balance_degree: HoistRopeBalanceDegree,
    pub hoist_rope_twisting_method: HoistRopeTwistingMethod,
}
//
impl UserSelect {
    ///
    /// Метод чтения файла Json
    /// - 'file_path' - путь к файлу
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(file_path)?;
        let data: UserSelect = serde_json::from_str(&file)?;
        Ok(data)
    }
}