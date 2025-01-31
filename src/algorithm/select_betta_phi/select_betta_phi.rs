use crate::{
    algorithm::{
        context::{context::Context, ctx_result::CtxResult},
        entities::{bet_phi::BetPhi, lifting_class::LiftClass},
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use std::sync::{Arc, RwLock};
///
/// Struct, that make choice β2 and ϕ2 coefficients, based on user [lifting class](design\docs\algorithm\part01\initial_data.md)
/// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct SelectBettaPhi {
    dbgid: DbgId,
    /// [BetPhi] instance, where store value of coefficients β2 and ϕ2
    value: Option<BetPhi>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Arc<RwLock<Context>>,
}
//
//
impl SelectBettaPhi {
    ///
    /// Class Constructor
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: Arc<RwLock<Context>>) -> Self {
        Self {
            dbgid: DbgId("SelectBetPhi".to_string()),
            value: None,
            ctx,
        }
    }
}
//
//
impl Eval for SelectBettaPhi {
    ///
    /// Method make choice β2 and ϕ2 coefficients, based on user [lifting class](design\docs\algorithm\part01\initial_data.md)
    /// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, StrErr> {
        match &self.value {
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
                let result = match initial.lift_class {
                    LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
                    LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
                    LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
                    LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
                };
                self.value = Some(result.clone());
                match self.ctx.write() {
                    Ok(mut ctx) => {
                        ctx.bet_phi.result = CtxResult::Ok(result);
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
