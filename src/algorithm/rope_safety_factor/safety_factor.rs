use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult}};
use super::{safety_factor_ctx::SafetyFactorCtx, select_safety_coeff::SelectSafetyCoeff};
///
/// Calculation step: [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct SafetyFactor {
    dbg: DbgId,
    /// value of [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<SafetyFactorCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>
}
//
//
impl SafetyFactor {
    ///
    /// New instance [SafetyFactor]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbg: DbgId("SafetyFactor".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for SafetyFactor {
    ///
    /// Method of calculating [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            let result = match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let mechanism_work_type = initial.mechanism_work_type.clone();
                    let winding_type = initial.winding_type;
                    let mark_fire_exp_env = initial.mark_fire_exp_env;
                    let crane_work_area = initial.crane_work_area.clone();
                    let result = SelectSafetyCoeff::new(
                        &self.dbg,
                        winding_type,
                        mark_fire_exp_env,
                        crane_work_area,
                        mechanism_work_type,
                    ).eval();
                    let result = match result {
                        Ok(coeff) => coeff,
                        Err(err) => return CtxResult::Err(StrErr(format!(
                            "{}.eval | Select safety factor coefficient error: {:?}",
                            self.dbg, err
                        ))),
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
impl std::fmt::Debug for SafetyFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SafetyFactor")
            .field("dbgid", &self.dbg)
            .field("value", &self.value)
            .finish()
    }
}
