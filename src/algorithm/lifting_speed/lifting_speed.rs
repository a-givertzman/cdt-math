use futures::future::BoxFuture;
use crate::{
    algorithm::{
        context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult},
        entities::{driver_type::DriverType, loading_combination::LoadingCombination}, initial_ctx::initial_ctx::InitialCtx,
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, sync::switch::Switch, types::eval_result::EvalResult},
};
use super::lifting_speed_ctx::LiftingSpeedCtx;
///
/// Calculation step: [steady-state lifting speed of the load](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct LiftingSpeed<'a> {
    dbg: DbgId,
    /// value of [steady-state lifting speed](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<LiftingSpeedCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Switch, EvalResult> + Send + 'a>,
}
//
//
impl<'a> LiftingSpeed<'a> {
    ///
    /// New instance [LiftingSpeed]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<'a, Switch, EvalResult> + Send + 'a) -> Self {
        Self {
            dbg: DbgId("LiftingSpeed".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Method returns half of the speed
    /// [reference to calculating documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'vhmax' - nominal lifting speed of the mechanism
    fn vhmax_half(vhmax: f64) -> f64 {
        vhmax * 0.5
    }
}
//
//
impl<'b, 'a:'b> Eval<'a, Switch, EvalResult> for LiftingSpeed<'b> {
    ///
    /// Method of calculating the steady-state lifting speed of the load
    /// [reference to steady-state lifting speed choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&'a mut self, switch: Switch) -> BoxFuture<'a, EvalResult> {
        let result = Box::pin(async {
            let (switch, result) = self.ctx.eval(switch).await;
            (switch, match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let result = match initial.load_comb {
                        LoadingCombination::A1 | LoadingCombination::B1 => match initial.driver_type {
                            DriverType::Hd1 => initial.vhmax,
                            DriverType::Hd2 | DriverType::Hd3 => initial.vhcs,
                            DriverType::Hd4 => Self::vhmax_half(initial.vhmax),
                            DriverType::Hd5 => 0.0,
                        },
                        LoadingCombination::C1 => match initial.driver_type {
                            DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => initial.vhmax,
                            DriverType::Hd3 | DriverType::Hd5 => Self::vhmax_half(initial.vhmax),
                        },
                    };
                    let result = LiftingSpeedCtx {
                        result: result,
                    };
                    self.value = Some(result.clone());
                    ctx.write(result)
                }
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbg, err
                ))),
                CtxResult::None => CtxResult::None,
            })
        });
        log::debug!("{}.eval | Done", self.dbg);
        result
    }
}
//
//
impl std::fmt::Debug for LiftingSpeed<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiftingSpeed")
            .field("dbgid", &self.dbg)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
