use crate::{algorithm::{context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult}, entities::hook::Hook}, kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, str_err::str_err::StrErr}};
use super::user_hook_ctx::UserHookCtx;
///
/// Represents user hook and make request to user for choosing one
pub struct UserHook {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHookCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
    req: Request<Hook>,
}
//
//
impl UserHook {
    ///
    /// New instance [UserHook]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<Hook>, ctx: impl Eval + 'static) -> Self{
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
impl Eval for UserHook {
    fn eval(&mut self) -> CtxResult<Context, StrErr> {
        match self.ctx.eval() {
            CtxResult::Ok(ctx) => {
                let reply = self.req.fetch(&ctx);
                let result = UserHookCtx { result: reply };
                self.value = Some(result.clone());
                ctx.write(result)
            },
            CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                "{}.eval | Read context error: {:?}",
                self.dbgid, err
            ))),
            CtxResult::None => todo!(),
        }

    }
}
//
//
impl std::fmt::Debug for UserHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHook")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}