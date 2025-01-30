use crate::{algorithm::context::ctx_result::CtxResult, kernel::str_err::str_err::StrErr};
///
/// Struct to store result of algorithm, that calculate steady state lifting speed
/// - 'result' - value of steady state lifting speed
#[derive(Debug, Clone)]
pub struct LiftingSpeedCtx {
    pub result: CtxResult<f64, StrErr>,
}
//
//
impl Default for LiftingSpeedCtx {
    fn default() -> Self {
        Self {
            result: CtxResult::default(), 
        }
    }
}