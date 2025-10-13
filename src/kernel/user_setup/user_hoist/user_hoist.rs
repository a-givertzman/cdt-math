use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::{bearing_filter::bearing_filter_ctx::BearingFilterCtx, context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoist_filter::hoist_filter_ctx::HoistFilterCtx},
    infrostructure::client::choose_user_hoist::ChooseUserHoistReply,
    kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, types::eval_result::EvalResult, user_setup::user_hoist::user_hoist_ctx::UserHoistCtx},
};
///
/// Represents user hoist and make request to user for choosing one
pub struct UserHoist {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHoistCtx>,
    /// Event interface
    req: Request<HoistFilterCtx, ChooseUserHoistReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl UserHoist {
    ///
    /// New instance [UserHoist]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<HoistFilterCtx, ChooseUserHoistReply>, ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self { 
            dbgid: DbgId("UserHoist".to_string()), 
            value: None,
            req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for UserHoist {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let variants: &HoistFilterCtx = ctx.read();
                    let variants = variants.to_owned();
                    let reply = self.req.fetch(variants).await;
                    let result = UserHoistCtx { result: reply.answer };
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
impl std::fmt::Debug for UserHoist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHoist")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}
