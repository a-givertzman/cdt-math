use futures::future::BoxFuture;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, sync::switch::Switch, types::eval_result::EvalResult}};
use super::rope_effort_ctx::RopeEffortCtx;
///
/// Calculation step: [rope effort](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct RopeEffort<'a> {
    dbgid: DbgId,
    /// value of [rope effort](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    value: Option<RopeEffortCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<Switch, EvalResult> + Send + 'a>,
}
//
//
impl<'a> RopeEffort<'a> {
    ///
    /// New instance [RopeEffort]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<Switch, EvalResult> + Send + 'a) -> Self {
        Self {
            dbgid: DbgId("RopeEffort".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Switch, EvalResult> for RopeEffort<'_> {
    ///
    /// Method of calculating [rope effort](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md), based on user loading capacity
    fn eval(&'_ mut self, switch: Switch) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let (switch, result) = self.ctx.eval(switch).await;
            let result = match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let result = match initial.load_capacity {
                        x if x <= 1.0 => 7.5,
                        x if x <= 2.0 => 10.0,
                        x if x <= 6.0 => 20.0,
                        x if x <= 10.0 => 30.0,
                        x if x <= 15.0 => 40.0,
                        x if x <= 20.0 => 50.0,
                        x if x <= 40.0 => 60.0,
                        x if x <= 100.0 => 90.0,
                        x if x <= 150.0 => 130.0,
                        x if x <= 200.0 => 180.0,
                        x if x <= 500.0 => 220.0,
                        _ => return (switch, CtxResult::Err(StrErr(format!(
                            "{}.eval | Error value of user loading capacity",
                            self.dbgid
                        ))))
                    };
                    let result = RopeEffortCtx {
                        result,
                    };
                    ctx.write(result)
                }
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbgid, err
                ))),
                CtxResult::None => CtxResult::None,
            };
            (switch, result)
        })
    }
}
//
//
impl std::fmt::Debug for RopeEffort<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RopeEffort")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}
