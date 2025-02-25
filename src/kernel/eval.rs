use async_trait::async_trait;
use super::str_err::str_err::StrErr;
use crate::algorithm::context::ctx_result::CtxResult;
///
/// Trate defines common evaluation function for calculations classes
#[async_trait]
pub trait Eval<Out> {
    ///
    /// Pervorms a calculation
    /// - Returns [Out] contains results inside
    async fn eval(&mut self) -> CtxResult<Out, StrErr>;
}
