use futures::future::BoxFuture;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, maximum_force::max_force_ctx::MaxForceCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, types::eval_result::EvalResult}};
use super::min_break_force_ctx::MinBreakForceCtx;
///
/// Calculation step: [minimum breaking force](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct MinBreakForce {
    dbgid: DbgId,
    /// value of [minimum breaking force](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<MinBreakForceCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl MinBreakForce {
    ///
    /// New instance [MinBreakForce]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("MinBreakForce".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for MinBreakForce {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let max_force = ContextRead::<MaxForceCtx>::read(&ctx).result.clone();
                    let rope_safety = todo!("Create context calculation step `RopeSafety`");
                    let result = MinBreakForceCtx {
                        result: max_force * rope_safety,
                    };
                    self.value = Some(result.clone());
                    ctx.write(result)
                }
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
impl std::fmt::Debug for MinBreakForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinBreakForce")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}