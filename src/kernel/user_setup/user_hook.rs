use futures::future::BoxFuture;
use crate::{
    algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hook_filter::hook_filter_ctx::HookFilterCtx},
    infrostructure::client::choose_user_hook::ChooseUserHookReply,
    kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, str_err::str_err::StrErr, sync::switch::Switch, types::eval_result::EvalResult},
};
use super::user_hook_ctx::UserHookCtx;
///
/// Represents user hook and make request to user for choosing one
pub struct UserHook<'a> {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHookCtx>,
    /// Event interface
    req: Request<'a, HookFilterCtx, ChooseUserHookReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Switch, EvalResult> + Send + 'a>,
}
//
//
impl<'a> UserHook<'a> {
    ///
    /// New instance [UserHook]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<'a, HookFilterCtx, ChooseUserHookReply>, ctx: impl Eval<'a, Switch, EvalResult> + Send + 'a) -> Self{
        Self { 
            dbgid: DbgId("UserHook".to_string()), 
            value: None,
            req: req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<'b, 'a:'b> Eval<'a, Switch, EvalResult> for UserHook<'b> {
    fn eval(&'a mut self, mut switch: Switch) -> BoxFuture<'a, EvalResult> {
        let link = switch.link();
        Box::pin(async {
            let (switch, result) = self.ctx.eval(switch).await;
            (switch, match result {
                CtxResult::Ok(ctx) => {
                    let variants: &HookFilterCtx = ctx.read();
                    let reply = self.req.fetch(variants.to_owned(), link).await;
                    let result = UserHookCtx { result: reply.choosen };
                    self.value = Some(result.clone());
                    ctx.write(result)
                },
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbgid, err
                ))),
                CtxResult::None => CtxResult::None,
            })
        })
    }
}
//
//
impl std::fmt::Debug for UserHook<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHook")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}