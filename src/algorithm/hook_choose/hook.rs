use crate::kernel::dbgid::dbgid::DbgId;


///
/// Класс, для хранения информации об крюке
/// - 'GOST' - ГОСТ номер крюка
/// - 'ISO_4301' - порядковый номер ИСО крюка
/// - 'mechanism_work_type' - режим работы механизма согласно ГОСТ 34017-2016
/// - 'hook_type' - тип крюка
/// - 'd_tail' - диаметр хвостовика крюка под подшипник
#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Hook{
    dbgid: DbgId,
    ISO_4301: String,
    mechanism_work_type: String,
    hook_type: String,
    max_m_to_lift: f64,
    pub d_tail: f64,
}
//
//
//
impl Hook{
    ///
    /// Конструктор класса Hook
    pub fn new(ISO_4301: String, mechanism_work_type: String, hook_type: String, max_m_to_lift: f64, d_tail: f64) -> Self{
        Self { dbgid: DbgId(format!("Hook")), 
                ISO_4301, 
                mechanism_work_type, 
                hook_type, 
                max_m_to_lift: max_m_to_lift,
                d_tail 
             }

    }
    ///
    /// Метод вывода крюка в консоль
    pub fn print(&self){
        log::debug!("{}", format!("{}.print | ISO_4301: {}",self.dbgid,self.ISO_4301));
        log::debug!("{}", format!("{}.print | mechanism_work_type: {}",self.dbgid,self.mechanism_work_type));
        log::debug!("{}", format!("{}.print | hook_type: {}",self.dbgid,self.hook_type));
        log::debug!("{}", format!("{}.print | d_tail: {}",self.dbgid,self.d_tail));
    }
}