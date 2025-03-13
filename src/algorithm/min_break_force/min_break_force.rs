use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;

use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, maximum_force::max_force_ctx::MaxForceCtx, rope_safety_factor::safety_factor_ctx::SafetyFactorCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult}};
use super::min_break_force_ctx::MinBreakForceCtx;
///
/// Calculation step: [minimum required breaking force in rope]](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct MinBreakForce {
    dbg: DbgId,
    /// value of [minimum required breaking force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<MinBreakForceCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>
}
//
//
impl MinBreakForce {
    ///
    /// New instance [MinBreakForce]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbg: DbgId("MinBreakForce".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for MinBreakForce {
    ///
    /// Method of calculating [minimum required breaking force in rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            let result = match result {
                CtxResult::Ok(ctx) => {
                    let max_force = ContextRead::<MaxForceCtx>::read(&ctx).result;
                    let safety_factor = ContextRead::<SafetyFactorCtx>::read(&ctx).result;
                    let result = MinBreakForceCtx {
                        result: max_force * safety_factor,
                    };
                    ctx.write(result)
                }
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbg, err
                ))),
                CtxResult::None => CtxResult::None,
            };
            result
        })
    }
}
//
//
impl std::fmt::Debug for MinBreakForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinBreakForce")
            .field("dbgid", &self.dbg)
            .field("value", &self.value)
            .finish()
    }
}
