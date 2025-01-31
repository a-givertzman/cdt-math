use crate::{
    algorithm::{context::{context::Context, ctx_result::CtxResult}, entities::{driver_type::DriverType, loading_combination::LoadingCombination}},
    kernel::{
        dbgid::dbgid::DbgId,
        eval::Eval,
        str_err::str_err::StrErr,
    },
};
use std::sync::{Arc, RwLock};
///
/// Сlass, that select the steady-state lifting speed of the load
/// [reference to steady-state lifting speed documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - value of steady-state lifting speed
/// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
#[derive(Debug, Clone)]
pub struct LiftingSpeed {
    dbgid: DbgId,
    value: Option<f64>,
    ctx: Arc<RwLock<Context>>,
}
//
//
impl LiftingSpeed {
    ///
    /// Class Constructor
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: Arc<RwLock<Context>>) -> Self {
        Self {
            dbgid: DbgId("LiftingSpeed".to_string()),
            value: None,
            ctx,
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
    fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, StrErr> {
        match self.value {
            Some(_) => CtxResult::Ok(self.ctx.clone()),
            None => {
                let initial = match self.ctx.read() {
                    Ok(ctx) => ctx.initial.clone(),
                    Err(err) => {
                        return CtxResult::Err(StrErr(format!(
                            "{}.eval | Read context error: {:?}",
                            self.dbgid, err
                        )))
                    }
                };
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
                self.value = Some(result);
                match self.ctx.write() {
                    Ok(mut ctx) => {
                        ctx.lifting_speed.result = CtxResult::Ok(result);
                        CtxResult::Ok(self.ctx.clone())
                    }
                    Err(err) => CtxResult::Err(StrErr(format!(
                        "{}.eval | Read context error: {:?}",
                        self.dbgid, err
                    ))),
                }
            }
        }
    }
}
