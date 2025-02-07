use crate::{
    algorithm::{
        context::{context::Context, ctx_result::CtxResult},
        entities::{bet_phi::BetPhi, lifting_class::LiftClass},
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use super::select_betta_phi_ctx::SelectBetPhiCtx;
///
/// Struct, that make choice β2 and ϕ2 coefficients, based on user [lifting class](design\docs\algorithm\part01\initial_data.md)
/// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct SelectBettaPhi {
    dbgid: DbgId,
    /// [BetPhi] instance, where store value of coefficients β2 and ϕ2
    value: Option<SelectBetPhiCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
}
//
//
impl SelectBettaPhi {
    ///
    /// Class Constructor
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval + 'static) -> Self {
        Self {
            dbgid: DbgId("SelectBetPhi".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval for SelectBettaPhi {
    ///
    /// Method make choice β2 and ϕ2 coefficients, based on user [lifting class](design\docs\algorithm\part01\initial_data.md)
    /// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Context, StrErr> {
        match self.ctx.eval() {
            CtxResult::Ok(mut ctx) => {
                let initial = ctx.initial.clone();
                let result = match initial.lift_class {
                    LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
                    LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
                    LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
                    LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
                };
                let result = SelectBetPhiCtx {
                    result: CtxResult::Ok(result),
                };
                self.value = Some(result.clone());
                ctx.select_bet_phi = result;
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
impl std::fmt::Debug for SelectBettaPhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiftingSpeed")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
