use crate::kernel::str_err::str_err::StrErr;
use super::{context::Context, ctx_result::CtxResult};
///
/// Provides restricted write access to the [Context] members
pub trait ContextWrite<T> {
    fn write(self, value: T) -> CtxResult<Context, StrErr>;
}