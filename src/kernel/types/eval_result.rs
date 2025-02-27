use crate::{
    algorithm::context::{context::Context, ctx_result::CtxResult},
    kernel::{str_err::str_err::StrErr, sync::switch::Switch},
};
///
/// Result returned from Calculation steps
pub type EvalResult = (Switch, CtxResult<Context, StrErr>);