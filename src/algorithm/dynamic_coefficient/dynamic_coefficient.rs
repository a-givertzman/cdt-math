use futures::future::BoxFuture;
use crate::{
    algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, lifting_speed::lifting_speed_ctx::LiftingSpeedCtx, select_betta_phi::select_betta_phi_ctx::SelectBetPhiCtx},
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, sync::switch::Switch, types::eval_result::EvalResult},
};
use super::dynamic_coefficient_ctx::DynamicCoefficientCtx;
///
/// Calculation step: [dynamic coefficient](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct DynamicCoefficient<'a> {
    dbgid: DbgId,
    /// value of [dynamic coefficient](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<DynamicCoefficientCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Switch, EvalResult> + Send + 'a>,
}
//
//
impl<'a> DynamicCoefficient<'a> {
    ///
    /// New instance [DynamicCoefficient]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<'a, Switch, EvalResult> + Send + 'a) -> Self {
        Self {
            dbgid: DbgId("DynamicCoefficient".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<'b, 'a:'b> Eval<'a, Switch, EvalResult> for DynamicCoefficient<'b> {
    ///
    /// Method of calculating the dynamic coefficient
    /// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&'a mut self, switch: Switch) -> BoxFuture<'a, EvalResult> {
        Box::pin(async {
            let (switch, result) = self.ctx.eval(switch).await;
            (switch, match result {
                CtxResult::Ok(ctx) => {
                    let result = match self.value.clone() {
                        Some(dynamic_coefficient) => dynamic_coefficient,
                        None => {
                            let lifting_speed = ContextRead::<LiftingSpeedCtx>::read(&ctx).result;
                            let bet_phi = ContextRead::<SelectBetPhiCtx>::read(&ctx).result;
                            let result = bet_phi.phi + bet_phi.bet * lifting_speed;
                            DynamicCoefficientCtx {
                                result: (result * 1000.0).round() / 1000.0,
                            }
                        }
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
impl std::fmt::Debug for DynamicCoefficient<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicCoefficient")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
