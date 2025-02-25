use futures::future::BoxFuture;
use crate::{
    algorithm::{
        context::{context::Context, context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult},
        entities::{bet_phi::BetPhi, lifting_class::LiftClass}, initial_ctx::initial_ctx::InitialCtx,
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use super::select_betta_phi_ctx::SelectBetPhiCtx;
///
/// Calculation step: [β2 and ϕ2 coefficients](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct SelectBettaPhi<'a> {
    dbgid: DbgId,
    /// [BetPhi] instance, where store value of coefficients β2 and ϕ2
    value: Option<SelectBetPhiCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Context> + Send + 'a>,
}
//
//
impl<'a> SelectBettaPhi<'a> {
    ///
    /// New instance [SelectBettaPhi]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<'a, Context> + Send + 'a) -> Self {
        Self {
            dbgid: DbgId("SelectBetPhi".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<'a> Eval<'a, Context> for SelectBettaPhi<'a> {
    ///
    /// Method make choice β2 and ϕ2 coefficients, based on user [lifting class](design\docs\algorithm\part01\initial_data.md)
    /// [reference to β2 and ϕ2 coefficients documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&'a mut self) -> BoxFuture<'a, CtxResult<Context, StrErr>> {
        Box::pin(async {
            match self.ctx.eval().await {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let result = match initial.lift_class {
                        LiftClass::Hc1 => BetPhi::new(0.17, 1.05),
                        LiftClass::Hc2 => BetPhi::new(0.34, 1.10),
                        LiftClass::Hc3 => BetPhi::new(0.51, 1.15),
                        LiftClass::Hc4 => BetPhi::new(0.68, 1.20),
                    };
                    let result = SelectBetPhiCtx { result };
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
impl std::fmt::Debug for SelectBettaPhi<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiftingSpeed")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
