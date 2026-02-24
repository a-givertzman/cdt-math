use futures::future::BoxFuture;
use sal_sync::services::entity::{dbg_id::DbgId, error::str_err::StrErr};
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoist_rope_filter::hoist_rope_filter_ctx::HoistRopeFilterCtx}, infrostructure::client::choose_hoisting_rope::ChooseHoistingRopeReply, kernel::{eval::Eval, request::Request, types::eval_result::EvalResult}};
use super::user_hoist_rope_ctx::UserHoistRopeCtx;
///
/// Represents user hoisting rope and make request to user for choosing one
pub struct UserHoistRope {
    dbgid: DbgId,
    /// value of user hoisting rope
    value: Option<UserHoistRopeCtx>,
    /// Event interface
    req: Request<HoistRopeFilterCtx, ChooseHoistingRopeReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl UserHoistRope {
    ///
    /// New instance [UserHoistRope]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<HoistRopeFilterCtx, ChooseHoistingRopeReply>, ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self { 
            dbgid: DbgId("UserHoistRope".to_string()), 
            value: None,
            req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for UserHoistRope {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let variants: &HoistRopeFilterCtx = ctx.read();
                    let variants = variants.to_owned();
                    let reply = self.req.fetch(variants).await;
                    let result = UserHoistRopeCtx { result: reply.answer };
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
impl std::fmt::Debug for UserHoistRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserHoistRope")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}
