use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::{
        context::{
            context_access::{
                ContextRead, 
                ContextWrite
            }, 
            ctx_result::CtxResult
        }, 
        hook_filter::hook_filter_ctx::HookBlockFilterCtx
    },
    infrostructure::client::choose_user_hook::ChooseUserHookReply,
    kernel::{
        dbgid::dbgid::DbgId, 
        eval::Eval, 
        request::Request, 
        types::eval_result::EvalResult
    },
};
use super::user_hook_ctx::UserHookCtx;
///
/// Represents user hook and make request to user for choosing one
pub struct UserHookBlock {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHookCtx>,
    /// Event interface
    req: Request<HookBlockFilterCtx, ChooseUserHookReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl UserHookBlock {
    ///
    /// New instance [UserHook]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<HookBlockFilterCtx, ChooseUserHookReply>, ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self{
        Self { 
            dbgid: DbgId("UserHookBlock".to_string()), 
            value: None,
            req: req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for UserHookBlock {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let variants: &HookBlockFilterCtx = ctx.read();
                    let reply = self.req.fetch(variants.to_owned()).await;
                    let result = UserHookCtx { result: reply.choosen };
                    self.value = Some(result.clone());
                    ctx.write(result)
                },
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbgid, err
                ))),
                CtxResult::None => CtxResult::None,
            }
        })
    }
}
//
//
impl std::fmt::Debug for UserHookBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHookBlock")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}