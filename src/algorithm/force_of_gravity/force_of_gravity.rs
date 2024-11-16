use crate::{algorithm::dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, kernel::{dbgid::dbgid::DbgId, str_err::str_err::StrErr}};


pub struct ForceOfGravity{
    pub(crate) dbgid: DbgId,
    pub(crate) dynamic_coefficient: DynamicCoefficient,
    pub(crate) m_to_lift: f64,
    pub(crate) g: f64,
    pub(crate) value: Option<f64>,
}
//
//
//
impl ForceOfGravity{
    ///
    /// Конструктор класса ForceOfGravity
    pub fn new(dynamic_coefficient: DynamicCoefficient, m_to_lift: f64) -> Self{
        Self { dbgid: DbgId(format!("ForceOfGravity")), dynamic_coefficient, m_to_lift, g: 9.81, value: None }
    }
    ///
    /// Метод расчёта силы тяжести, действующей на крюк
    pub fn eval(&mut self) -> Result<f64, StrErr> {
        match self.value {
            Some(val) => Ok(val),
            None => {
                match self.dynamic_coefficient.eval(){
                    Ok(value) => {
                        let fmg = value * self.m_to_lift * self.g;
                        self.value = Some(fmg);
                        Ok(fmg)
                    },
                    Err(err) => Err(err),
                }
            },
        }
    }
}