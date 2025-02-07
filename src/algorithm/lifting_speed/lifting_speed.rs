use crate::{
    algorithm::{
        context::{context::Context, context_access::ContextWrite, ctx_result::CtxResult},
        entities::{driver_type::DriverType, loading_combination::LoadingCombination},
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use super::lifting_speed_ctx::LiftingSpeedCtx;
///
/// Сlass, that select the steady-state lifting speed of the load
/// [reference to steady-state lifting speed documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct LiftingSpeed {
    dbgid: DbgId,
    /// value of [steady-state lifting speed](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<LiftingSpeedCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
}
//
//
impl LiftingSpeed {
    ///
    /// Class Constructor
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval + 'static) -> Self {
        Self {
            dbgid: DbgId("LiftingSpeed".to_string()),
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
impl Eval for LiftingSpeed {
    ///
    /// Method of calculating the steady-state lifting speed of the load
    /// [reference to steady-state lifting speed choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Context, StrErr> {
        match self.ctx.eval() {
            CtxResult::Ok(ctx) => {
                let initial = ctx.initial.clone();
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
                self.dbgid, err
            ))),
            CtxResult::None => CtxResult::None,
        }
    }
}
//
//
impl std::fmt::Debug for LiftingSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiftingSpeed")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
