use crate::{algorithm::context::{context::Context, ctx_result::CtxResult}, kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, str_err::str_err::StrErr}};
use super::user_hook_ctx::UserHookCtx;
///
/// Struct to store user hook and make request to user for choosing one
pub struct UserHook<T> {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHookCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
    req: Request<T>,
}
//
//
impl<T> UserHook<T> {
    pub fn new(req: Request<T>, ctx: impl Eval + 'static) -> Self{
        Self { 
            dbgid: DbgId("UserHook".to_string()), 
            value: None,
            ctx: Box::new(ctx),
            req: req,
        }
    }
}
//
//
impl<T> Eval for UserHook<T> {
    fn eval(&mut self) -> CtxResult<Context, StrErr> {
        todo!()
    }
}
//
//
impl<T> std::fmt::Debug for UserHook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHook")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}