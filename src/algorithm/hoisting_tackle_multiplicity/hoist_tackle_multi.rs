use futures::future::BoxFuture;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoisting_tackle::hoisting_tackle_ctx::HoistingTackleCtx, rope_count::rope_count_ctx::RopeCountCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, types::eval_result::EvalResult}};
use super::hoist_tackle_multi_ctx::HoistTackleMultiCtx;
///
/// Calculation step: [hoisting tackle multiplicity](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
pub struct HoistTackleMulti {
    dbgid: DbgId,
    /// value of [hoisting tackle multiplicity](design\docs\algorithm\part02\chapter_03_choose_hoisting_tackle.md)
    value: Option<HoistTackleMultiCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl HoistTackleMulti {
    ///
    /// New instance [HoistTackleMulti]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("HoistTackleMulti".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for HoistTackleMulti {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let hoisting_tackle = ContextRead::<HoistingTackleCtx>::read(&ctx).result.clone();
                    let rope_count = ContextRead::<RopeCountCtx>::read(&ctx).result.clone();
                    let result = HoistTackleMultiCtx {
                        result: rope_count / hoisting_tackle as f64,
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
impl std::fmt::Debug for HoistTackleMulti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoistTackleMulti")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}