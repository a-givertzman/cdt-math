use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::{
        choice_usage_class::usage_class_ctx::ChoiceUsageClassCtx, context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, entities::{hoist_group::HoistGroup, usage_class::UsageClass}, initial_ctx::initial_ctx::InitialCtx
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult},
};
///
/// Calculation step: [usage class](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct ChoiceUsageClass {
    dbgid: DbgId,
    /// [UsageClassCtx] instance, where store value of usage class
    value: Option<ChoiceUsageClassCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl ChoiceUsageClass {
    ///
    /// New instance [ChoiceUsageClass]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("SelectBetPhi".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for ChoiceUsageClass {
    ///
    /// Method make choice usage class, based on user [hoist_group](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let result = match initial.hoist_group {
                        HoistGroup::M1 => UsageClass::T0,
                        HoistGroup::M2 => UsageClass::T1,
                        HoistGroup::M3 => UsageClass::T2,
                        HoistGroup::M4 => UsageClass::T3,
                        HoistGroup::M5 => UsageClass::T4,
                        HoistGroup::M6 => UsageClass::T5,
                        HoistGroup::M7 => UsageClass::T6,
                        HoistGroup::M8 => UsageClass::T7,
                        HoistGroup::M9 => UsageClass::T8,
                    };
                    let result = ChoiceUsageClassCtx { result };
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
impl std::fmt::Debug for ChoiceUsageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChoiceUsageClass")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
