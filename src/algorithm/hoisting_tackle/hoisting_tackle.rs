use futures::future::BoxFuture;

use crate::{algorithm::context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult}, infrostructure::client::change_hoisting_tackle::ChangeHoistingTackleReply, kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, str_err::str_err::StrErr, sync::link::Link}};

use super::hoisting_tackle_ctx::HoistingTackleCtx;

///
/// Represents hoisting tackle and make request to user for changing one
pub struct HoistingTackle<'a> {
    dbgid: DbgId,
    /// value of hoisting tackle
    value: Option<HoistingTackleCtx>,
    /// Event interface
    link: Link,
    req: Request<'a, ChangeHoistingTackleReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Context> + Send + 'a>,
}
//
//
impl<'a> HoistingTackle<'a> {
    ///
    /// New instance [HoistingTackle]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(link: Link, req: Request<'a, ChangeHoistingTackleReply>, ctx: impl Eval<'a, Context> + Send + 'a) -> Self {
        Self { 
            dbgid: DbgId("HoistingTackle".to_string()), 
            value: None,
            link,
            req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<'a> Eval<'a, Context> for HoistingTackle<'a> {
    fn eval(&'a mut self) -> BoxFuture<'a, CtxResult<Context, StrErr>> {
        Box::pin(async {
            match self.ctx.eval().await {
                CtxResult::Ok(ctx) => {
                    let reply = self.req.fetch(ctx.clone(), &mut self.link).await;
                    let result = HoistingTackleCtx { result: reply.answer };
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
impl std::fmt::Debug for HoistingTackle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoistingTackle")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}