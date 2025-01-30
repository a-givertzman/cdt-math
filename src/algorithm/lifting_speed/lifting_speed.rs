use crate::{
    algorithm::context::{context::Context, ctx_result::CtxResult},
    kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, loading_combination::LoadingCombination},
    str_err::str_err::StrErr},
};
///
/// Сlass, that select the steady-state lifting speed of the load
/// [reference to steady-state lifting speed documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - value of steady-state lifting speed
/// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    dbgid: DbgId,
    value: Option<f64>,
    ctx: Context,
}
//
//
impl LiftingSpeed {
    ///
    /// Class Constructor
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: Context) -> Self {
        Self {
            dbgid: DbgId("LiftingSpeed".to_string()),
            value: None,
            ctx
        }
    }
    ///
    /// Method returns half of the speed
    /// [reference to calculating documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'vhmax' - nominal lifting speed of the mechanism
    fn vhmax_half(vhmax: f64) -> f64 {
        vhmax * 0.5
    }
    ///
    /// Method of calculating the steady-state lifting speed of the load
    /// [reference to steady-state lifting speed choice documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn eval(&mut self) -> f64 {
        match self.value {
            Some(lifting_speed) => return lifting_speed,
            None => {
                let result = match self.ctx.initial.load_comb {
                    LoadingCombination::A1 | LoadingCombination::B1 => match self.ctx.initial.driver_type {
                        DriverType::Hd1 => self.ctx.initial.vhmax,
                        DriverType::Hd2 | DriverType::Hd3 => self.ctx.initial.vhcs,
                        DriverType::Hd4 => Self::vhmax_half(self.ctx.initial.vhmax),
                        DriverType::Hd5 => 0.0,
                    },
                    LoadingCombination::C1 => match self.ctx.initial.driver_type {
                        DriverType::Hd1 | DriverType::Hd2 | DriverType::Hd4 => self.ctx.initial.vhmax,
                        DriverType::Hd3 | DriverType::Hd5 => Self::vhmax_half(self.ctx.initial.vhmax),
                    },
                };
                self.value = Some(result);
                self.ctx.lifting_speed.result = CtxResult::Ok(result);
                result
            },
        }

    }
}
