use std::str::FromStr;
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов рабочей местности крана
#[derive(Debug, Clone)]
pub enum CraneWorkArea {
    Normal,
    Aggressive,
    StrongAggresive,

}
//
//
//
impl FromStr for CraneWorkArea {
    ///
    /// Метод перевод из строки в тип перечисления CraneWorkArea
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "обычная" => Ok(Self::Normal),
            "агрессивная" => Ok(Self::Aggressive),
            "сильно агрессивная" => Ok(Self::StrongAggresive),
            _ => Err(format!("CraneWorkArea.from_str | Invalid CraneWorkArea: {}", s).into()),
        }
    }
}