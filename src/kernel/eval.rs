use futures::future::BoxFuture;
use super::str_err::str_err::StrErr;
use crate::algorithm::context::ctx_result::CtxResult;
///
/// Trate defines common evaluation function for calculations classes
pub trait Eval<Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval<'a>(&'a mut self) -> BoxFuture<'a, CtxResult<Out, StrErr>>;
}

