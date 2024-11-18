use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов компбинации нагрузок
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoadCombination {
    A1,
    B1,
    C1,
}
//
impl FromStr for LoadCombination {
    ///
    /// Метод перевод из строки в тип перечисления LoadCombination
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a1" => Ok(Self::A1),
            "b1" => Ok(Self::B1),
            "c1" => Ok(Self::C1),
            _ => Err(format!("LoadCombination.from_str | Invalid Cargo???LoadCombination: {}", s).into()),
        }
    }
}