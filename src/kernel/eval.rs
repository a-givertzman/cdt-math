use super::str_err::str_err::StrErr;
use crate::algorithm::context::{context::Context, ctx_result::CtxResult};
use std::sync::{Arc, RwLock};
///
/// Trate defines common evaluation function for calculations classes
pub trait Eval {
    ///
    /// Pervorms a calculation
    /// - Returns [Context] contains result inside
    fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, StrErr>;
}
