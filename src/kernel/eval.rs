use futures::future::BoxFuture;
use super::str_err::str_err::StrErr;
use crate::algorithm::context::ctx_result::CtxResult;
///
/// Trate defines common evaluation function for calculations classes
// #[async_trait]
pub trait Eval<'a, Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval(&'a mut self) -> BoxFuture<'a, CtxResult<Out, StrErr>>;
}

