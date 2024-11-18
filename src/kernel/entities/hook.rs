use crate::kernel::dbgid::dbgid::DbgId;
use serde::{Deserialize, Serialize};

use super::mechanism_work_type::MechanismWorkType;
///
/// Класс, для хранения информации о крюке
/// - 'hook_type' - тип крюка
/// - 'standard' - ГОСТ номер
/// - 'mechanism_work_type' - тип работы механизма подъема (enum [MechanismWorkType])
/// - 'load_capacity' - грузоподъёмность крюка
/// - 'shank_diameter' - диаметр хвостовика крюка под подшипник
/// - 'weight' - масса крюка
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hook {
    dbgid: DbgId,
    hook_type: String,
    standard: String,
    sequence_number: u8,
    capacity_M1: f64,
    shank_diameter: f64,
    weight: f64,
}
//
impl Hook{
    ///
    /// Конструктор класса Hook
    pub fn new() -> Self{
        Self { 
                dbgid: DbgId(format!("Hook")),
                hook_type: String::new(),
                standard: String::new(), 
                sequence_number: 0,
                capacity_M1: 0.0, 
                shank_diameter: 0.0, 
                weight: 0.0,
        }
    }
    ///
    /// Метод вывода информации о крюке в консоль
    pub fn paint(&self){
        log::debug!("{}", format!("{}.paint | Hook type: {}",self.dbgid,self.hook_type));
        log::debug!("{}", format!("{}.paint | Hook standard: {}",self.dbgid,self.standard));
        log::debug!("{}", format!("{}.paint | Hook sequence number: {}",self.dbgid,self.sequence_number));
        log::debug!("{}", format!("{}.paint | Hook load capacity M1: {:?}",self.dbgid,self.capacity_M1));
        log::debug!("{}", format!("{}.paint | Hook shank diameter: {}",self.dbgid,self.shank_diameter));
        log::debug!("{}", format!("{}.paint | Hook weight: {}",self.dbgid,self.weight));

    }
}