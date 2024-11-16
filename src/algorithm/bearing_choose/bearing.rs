use crate::kernel::dbgid::dbgid::DbgId;
#[derive(Debug, Clone)]
pub struct Bearing{
    pub(crate) dbgid: DbgId,
    pub(crate) name: String,
    pub(crate) d_out: f64
}
//
//
//
impl Bearing{
    ///
    /// Конструктор класса Bearing
    pub fn new(name: String, d_out: f64)->Self{
        Self { dbgid: DbgId(format!("Bearing")),name, d_out }
    }
    ///
    /// Метод вывода подшипника в консоль
    pub fn print(&self){
        log::debug!("{}", format!("{}.print | Bearing name: {}",self.dbgid,self.name));
        log::debug!("{}", format!("{}.print | Bearing out diameter: {}",self.dbgid,self.d_out));

    }
}