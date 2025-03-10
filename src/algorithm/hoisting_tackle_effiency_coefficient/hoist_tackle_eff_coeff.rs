use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoisting_tackle_multiplicity::hoist_tackle_multi_ctx::HoistTackleMultiCtx, initial_ctx::initial_ctx::InitialCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult}};
use super::hoist_tackle_eff_coeff_ctx::HoistTackleEffCoeffCtx;
///
/// Calculation step: [hoisting tackle effiency coefficient](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct HoistTackleEffCoeff {
    dbgid: DbgId,
    /// value of [hoisting tackle effiency coefficient](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<HoistTackleEffCoeffCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl HoistTackleEffCoeff {
    ///
    /// [Efficiency of deflection rope pulleys](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    const N_DEFL_BLOCK: f64 = 0.985;
    ///
    /// [Efficiency of the bypass rope pulleys of the pulley system](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    const N_BLOCK: f64 = 0.98;
    ///
    /// New instance [HoistTackleEffCoeff]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("HoistTackleEffCoeff".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for HoistTackleEffCoeff {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let deflect_blocks_count = initial.deflect_blocks_count;
                    let hoist_tackle_multi = ContextRead::<HoistTackleMultiCtx>::read(&ctx).result.clone();
                    let result = HoistTackleEffCoeffCtx {
                        result: Self::N_DEFL_BLOCK.powf(deflect_blocks_count) *
                                (1.0 - Self::N_BLOCK.powf(hoist_tackle_multi)) / 
                                ((1.0 - Self::N_BLOCK) * hoist_tackle_multi),
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
impl std::fmt::Debug for HoistTackleEffCoeff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoistTackleEffCoeff")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}