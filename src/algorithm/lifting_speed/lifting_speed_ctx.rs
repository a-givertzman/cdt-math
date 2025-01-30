use crate::kernel::str_err::str_err::StrErr;

#[derive(Debug, Clone)]
pub struct LiftingSpeedCtx {
    pub result: Result<f64, StrErr>,
}
//
//
impl Default for LiftingSpeedCtx {
    fn default() -> Self {
        Self {
            result: Err(StrErr("LiftingSpeedCtx | Not Initialized yet".into())) 
        }
    }
}