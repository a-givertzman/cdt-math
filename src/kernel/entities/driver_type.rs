use std::str::FromStr;
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов привода мех.под.
#[derive(Debug, Clone)]
pub enum DriverType {
    Hd1,
    Hd2,
    Hd3,
    Hd4,
    Hd5
}
//
//
//
impl FromStr for DriverType {
    ///
    /// Метод перевод из строки в тип перечисления DriverType
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hd1" => Ok(Self::Hd1),
            "hd2" => Ok(Self::Hd2),
            "hd3" => Ok(Self::Hd3),
            "hd4" => Ok(Self::Hd4),
            "hd5" => Ok(Self::Hd5),
            _ => Err(format!("DriverType.from_str | Invalid Cargo???DriverType: {}", s).into()),
        }
    }
}