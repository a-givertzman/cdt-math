use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{algorithm::{constants::common, context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoisting_tackle_effiency_coefficient::hoist_tackle_eff_coeff_ctx::HoistTackleEffCoeffCtx, initial_ctx::initial_ctx::InitialCtx, load_hand_device_mass::load_hand_device_mass_ctx::LoadHandDeviceMassCtx, rope_count::rope_count_ctx::RopeCountCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult}};
use super::max_force_ctx::MaxForceCtx;
///
/// Calculation step: [maximum force in hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct MaxForce {
    dbgid: DbgId,
    /// value of [maximum force in hoisting rope](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<MaxForceCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl MaxForce {
    ///
    /// New instance [MaxForce]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("MaxForce".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for MaxForce {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let loading_capacity = initial.load_capacity;
                    let hoist_tackle_eff_coeff = ContextRead::<HoistTackleEffCoeffCtx>::read(&ctx).result.clone();
                    let rope_count = ContextRead::<RopeCountCtx>::read(&ctx).result.clone();
                    let total_mass = ContextRead::<LoadHandDeviceMassCtx>::read(&ctx).total_mass.clone();
                    let result = MaxForceCtx {
                        result: ((loading_capacity + total_mass) * common::G) /
                                (rope_count * hoist_tackle_eff_coeff),
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
impl std::fmt::Debug for MaxForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaxForce")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}