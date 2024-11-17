///
/// Класс, для хранение коэффициентов поелзного действия отклоняющих\обводных канатных блоков
pub struct CoeffDeflectingAndBypass{
    pub(crate) deflecting_coeff: f64,
    pub(crate) bypass_coeff: f64,
}
//
//
//
impl CoeffDeflectingAndBypass{
    ///
    /// Конструктор класса CoeffDeflectingAndBypass
    pub fn new() -> Self{
        Self { deflecting_coeff: 0.985, bypass_coeff: 0.98 }
    }
}