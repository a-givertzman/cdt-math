use crate::{algorithm::context::ctx_result::CtxResult, kernel::str_err::str_err::StrErr};

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