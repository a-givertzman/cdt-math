use super::{context::Context, ctx_result::CtxResult};
use crate::{
    algorithm::dynamic_coefficient::dynamic_coefficient_ctx::DynamicCoefficientCtx,
    kernel::str_err::str_err::StrErr,
};
///
/// Provides restricted write access to the [Context] members
pub trait ContextWrite<T> {
    fn write(self, value: T) -> CtxResult<Context, StrErr>;
}
///
/// Provides simple read access to the [Context] members
pub trait ContextRead<T> {
    fn read(&self) -> T;
}
//
//
impl ContextWrite<DynamicCoefficientCtx> for Context {
    fn write(mut self, value: DynamicCoefficientCtx) -> CtxResult<Self, StrErr> {
        self.dynamic_coefficient = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<DynamicCoefficientCtx> for Context {
    fn read(&self) -> DynamicCoefficientCtx {
        self.dynamic_coefficient.clone()
    }
}