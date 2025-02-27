use futures::future::BoxFuture;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx, load_hand_device_mass::load_hand_device_mass_ctx::LoadHandDeviceMassCtx, rope_effort::rope_effort_ctx::RopeEffortCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, sync::switch::Switch, types::eval_result::EvalResult}};
use super::rope_count_ctx::RopeCountCtx;
///
/// Calculation step: [rope count](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct RopeCount<'a> {
    dbgid: DbgId,
    /// value of [rope count](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    value: Option<RopeCountCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<Switch, EvalResult> + Send + 'a>,
}
//
//
impl<'a> RopeCount<'a> {
    ///
    /// New instance [RopeCount]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<Switch, EvalResult> + Send + 'a) -> Self {
        Self {
            dbgid: DbgId("RopeCount".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// [Rounded up value](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md) to recommended rope count
    fn round_up(n: f64) -> f64 {
        let recommended = [2.0, 4.0, 8.0, 12.0, 16.0];
        for &r in &recommended {
            if n <= r {
                return r;
            }
        }
        *recommended.last().unwrap()
    }
}
//
//
impl Eval<Switch, EvalResult> for RopeCount<'_> {
    fn eval(&'_ mut self, switch: Switch) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let (switch, result) = self.ctx.eval(switch).await;
            (switch, match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let hook_weight = ContextRead::<LoadHandDeviceMassCtx>::read(&ctx).total_mass.clone();
                    let rope_effort = ContextRead::<RopeEffortCtx>::read(&ctx).result.clone();
                    let result = Self::round_up((initial.load_capacity+hook_weight)/rope_effort);
                    let result = RopeCountCtx {
                        result,
                    };
                    self.value = Some(result.clone());
                    ctx.write(result)
                }
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
impl std::fmt::Debug for RopeCount<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RopeCount")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}