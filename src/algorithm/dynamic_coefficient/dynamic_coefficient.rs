use crate::{
    algorithm::context::{context::Context, ctx_result::CtxResult},
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use super::dynamic_coefficient_ctx::DynamicCoefficientCtx;
///
/// Сlass, that calculate dynamic coefficient
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct DynamicCoefficient {
    dbgid: DbgId,
    /// value of [dynamic coefficient](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<DynamicCoefficientCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
}
//
//
impl DynamicCoefficient {
    ///
    /// Class Constructor
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval + 'static) -> Self {
        Self {
            dbgid: DbgId("DynamicCoefficient".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval for DynamicCoefficient {
    ///
    /// Method of calculating the dynamic coefficient
    /// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Context, StrErr> {
        match self.ctx.eval() {
            CtxResult::Ok(mut ctx) => {
                let result = match self.value.clone() {
                    Some(dynamic_coefficient) => dynamic_coefficient,
                    None => {
                        let lifting_speed =
                            ctx.lifting_speed.result.clone().unwrap();
                        let bet_phi = ctx.select_bet_phi.result.clone().unwrap();
                        DynamicCoefficientCtx {
                            result: CtxResult::Ok(bet_phi.phi + bet_phi.bet * lifting_speed),
                        }
                    }
                };
                self.value = Some(result.clone());
                ctx.dynamic_coefficient = result;
                CtxResult::Ok(ctx)
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
impl std::fmt::Debug for DynamicCoefficient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicCoefficient")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
