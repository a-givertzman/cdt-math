use futures::future::BoxFuture;
use crate::{
    algorithm::context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult},
    infrostructure::client::choose_user_hook::ChooseUserHookReply,
    kernel::{dbgid::dbgid::DbgId, eval::Eval, sync::link::Link, request::Request, str_err::str_err::StrErr},
};
use super::user_hook_ctx::UserHookCtx;
///
/// Represents user hook and make request to user for choosing one
pub struct UserHook<'a> {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHookCtx>,
    /// Event interface
    link: Link,
    req: Request<'a, ChooseUserHookReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<Context> + Send + 'a>,
}
//
//
impl<'a> UserHook<'a> {
    ///
    /// New instance [UserHook]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(link: Link, req: Request<'a, ChooseUserHookReply>, ctx: impl Eval<Context> + Send + 'a) -> Self{
        Self { 
            dbgid: DbgId("UserHook".to_string()), 
            value: None,
            link,
            req: req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Context> for UserHook<'_> {
    fn eval<'a>(&'a mut self) -> BoxFuture<'a, CtxResult<Context, StrErr>> {
        Box::pin(async {
            match self.ctx.eval().await {
                CtxResult::Ok(ctx) => {
                    let reply = self.req.fetch(ctx.clone(), &mut self.link).await;
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
impl std::fmt::Debug for UserHook<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHook")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}