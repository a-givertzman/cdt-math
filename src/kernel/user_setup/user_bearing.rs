use async_trait::async_trait;

use crate::{
    algorithm::context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult},
    infrostructure::client::choose_user_bearing::ChooseUserBearingReply,
    kernel::{dbgid::dbgid::DbgId, eval::Eval, sync::link::Link, request::Request, str_err::str_err::StrErr},
};
use super::user_bearing_ctx::UserBearingCtx;
///
/// Represents user bearing and make request to user for choosing one
pub struct UserBearing {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserBearingCtx>,
    /// Event interface
    link: Link,
    req: Request<ChooseUserBearingReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<Context> + Send>,
}
//
//
impl UserBearing {
    ///
    /// New instance [UserBearing]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(link: Link, req: Request<ChooseUserBearingReply>, ctx: impl Eval<Context> + Send + 'static) -> Self {
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
#[async_trait]
impl Eval<Context> for UserBearing {
    async fn eval(&mut self) -> CtxResult<Context, StrErr> {
        match self.ctx.eval().await {
            CtxResult::Ok(ctx) => {
                let reply = self.req.fetch(&ctx, &mut self.link).await;
                let result = UserBearingCtx { result: reply.answer };
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
impl std::fmt::Debug for UserBearing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserBearing")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}