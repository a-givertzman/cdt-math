#[derive(Debug, Clone)]
///
/// Класс реализующий расчёт суммарной и полезной масс
/// - 'summary_weight' - суммарная масса крюковой подвески
/// - 'good_weight' - полезная масса крюковой подвески
/// [reference to summary and good weights documentation](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md) 
pub struct SummaryGoodWeights{
    pub(crate) summary_weight:f64,
    pub(crate) good_weight: f64,
}
//
impl SummaryGoodWeights{
    ///
    /// Конструктор класса SummaryGoodWeights 
    pub fn new() -> Self{
        Self { summary_weight: 0.0, good_weight: 0.0 }
    }
    ///
    /// Метод расчёта полезной и суммарной масс
    /// - 'hook_m' - масса крюка, выбранного пользователем
    /// - 'weight_load_hand_device' - масса дополнительного грузозахватного органа
    /// - 'm_to_lift' - масса на крюке
    pub fn eval(&mut self,hook_m: f64,weight_load_hand_device: f64, m_to_lift: f64){
        self.summary_weight = hook_m + weight_load_hand_device;
        self.good_weight = m_to_lift - weight_load_hand_device;
    }
}