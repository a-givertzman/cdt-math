use crate::kernel::dbgid::dbgid::DbgId;

use super::summary_good_weights::summary_good_weight::SummaryGoodWeights;


///
/// Класс, для хранения информации об крюке
/// - 'GOST' - ГОСТ номер крюка
/// - 'ISO_4301' - порядковый номер ИСО крюка
/// - 'mechanism_work_type' - режим работы механизма согласно ГОСТ 34017-2016
/// - 'hook_type' - тип крюка
/// - 'd_tail' - диаметр хвостовика крюка под подшипник
/// - 'name_cargo_hand_device' - имя дополнительного грузозахватного органа
/// - 'weight_cargo_hand_device' - масса дополнительного грузозахватного органа
#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Hook{
    pub(crate) dbgid: DbgId,
    pub(crate) ISO_4301: String,
    pub(crate) mechanism_work_type: String,
    pub(crate) hook_type: String,
    pub(crate) max_m_to_lift: f64,
    pub(crate) hook_weight: f64,
    pub(crate) d_tail: f64,
    pub(crate) name_cargo_hand_device: String,
    pub(crate) weight_cargo_hand_device: f64,
    pub(crate) sum_good_weights: SummaryGoodWeights,

}
//
//
//
impl Hook{
    ///
    /// Конструктор класса Hook
    pub fn new(ISO_4301: String, mechanism_work_type: String, hook_type: String, max_m_to_lift: f64, d_tail: f64,name_cargo_hand_device: String, weight_cargo_hand_device: f64,hook_weight: f64) -> Self{
        Self { dbgid: DbgId(format!("Hook")), 
                ISO_4301, 
                mechanism_work_type, 
                hook_type, 
                max_m_to_lift: max_m_to_lift,
                d_tail,
                name_cargo_hand_device,
                weight_cargo_hand_device,
                sum_good_weights: SummaryGoodWeights::new(),
                hook_weight: hook_weight
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