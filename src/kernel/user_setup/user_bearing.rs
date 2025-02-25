use futures::future::BoxFuture;
use crate::{
    algorithm::context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult},
    infrostructure::client::choose_user_bearing::ChooseUserBearingReply,
    kernel::{dbgid::dbgid::DbgId, eval::Eval, sync::link::Link, request::Request, str_err::str_err::StrErr},
};
use super::user_bearing_ctx::UserBearingCtx;
///
/// Represents user bearing and make request to user for choosing one
pub struct UserBearing<'a> {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserBearingCtx>,
    /// Event interface
    link: Link,
    req: Request<'a, ChooseUserBearingReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Context> + Send + 'a>,
}
//
//
impl<'a> UserBearing<'a> {
    ///
    /// New instance [UserBearing]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(link: Link, req: Request<'a, ChooseUserBearingReply>, ctx: impl Eval<'a, Context> + Send + 'a) -> Self {
        Self { 
            dbgid: DbgId("UserBearing".to_string()), 
            value: None,
            link,
            req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<'a> Eval<'a, Context> for UserBearing<'a> {
    fn eval(&'a mut self) -> BoxFuture<'a, CtxResult<Context, StrErr>> {
        Box::pin(async {
            match self.ctx.eval().await {
                CtxResult::Ok(ctx) => {
                    let reply = self.req.fetch(ctx.clone(), &mut self.link).await;
                    let result = UserBearingCtx { result: reply.answer };
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
impl std::fmt::Debug for UserBearing<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserBearing")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}