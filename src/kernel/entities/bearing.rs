use serde::{Deserialize, Serialize};
use crate::kernel::dbgid::dbgid::DbgId;
///
/// Класс, для хранения информации о подшипнике
/// - 'name' - имя подшипника
/// - 'static_capacity' - статическая грузоподъемность
/// - 'outer_diameter' - наружный диаметр
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bearing {
    dbgid: DbgId,
    pub name: String,
    pub static_capacity: f64,
    pub outer_diameter: f64,
}
//
impl Bearing{
    ///
    /// Конструктор класса Hook
    pub fn new() -> Self{
        Self { 
                dbgid: DbgId(format!("Bearing")),
                name: String::new(),
                static_capacity: 0.0, 
                outer_diameter: 0.0, 
        }
    }
    ///
    /// Метод вывода информации о подшипнике в консоль
    pub fn paint(&self){
        log::debug!("{}", format!("{}.paint | Bearing name: {}",self.dbgid,self.name));
        log::debug!("{}", format!("{}.paint | Bearing static capacity: {}",self.dbgid,self.static_capacity));
        log::debug!("{}", format!("{}.paint | Bearing outer diameter: {}",self.dbgid,self.outer_diameter));
    }
}