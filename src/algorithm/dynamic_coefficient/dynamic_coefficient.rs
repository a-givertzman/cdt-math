use crate::{
    algorithm::context::{context::Context, ctx_result::CtxResult},
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use std::sync::{Arc, RwLock};
///
/// Сlass, that calculate dynamic coefficient
/// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct DynamicCoefficient {
    dbgid: DbgId,
    /// value of [dynamic coefficient](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<f64>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Arc<RwLock<Context>>,
}
//
//
impl DynamicCoefficient {
    ///
    /// Class Constructor
    /// - `ctx` - [Context]
    pub fn new(ctx: Arc<RwLock<Context>>) -> Self {
        Self {
            dbgid: DbgId("DynamicCoefficient".to_string()),
            value: None,
            ctx,
        }
    }
}
//
//
impl Eval for DynamicCoefficient {
    ///
    /// Method of calculating the dynamic coefficient
    /// [reference to dynamic coefficient documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, StrErr> {
        match self.value {
            Some(_) => CtxResult::Ok(self.ctx.clone()),
            None => match self.ctx.read() {
                Ok(ctx) => {
                    let lifting_speed = match &ctx.lifting_speed.result {
                        CtxResult::Ok(lifting_speed) => *lifting_speed,
                        CtxResult::Err(err) => {
                            return CtxResult::Err(StrErr(format!(
                                "{}.eval | Read context error: {:?}",
                                self.dbgid, err
                            )))
                        }
                        CtxResult::None => panic!("result is None"),
                    };
                    let bet_phi = match &ctx.bet_phi.result {
                        CtxResult::Ok(bet_phi) => bet_phi.clone(),
                        CtxResult::Err(err) => {
                            return CtxResult::Err(StrErr(format!(
                                "{}.eval | Read context error: {:?}",
                                self.dbgid, err
                            )))
                        }
                        CtxResult::None => panic!("result is None"),
                    };
                    let result = bet_phi.phi + bet_phi.bet * lifting_speed;
                    self.value = Some(result);
                    drop(ctx);
                    match self.ctx.write() {
                        Ok(mut ctx) => {
                            ctx.dynamic_coefficient.result = CtxResult::Ok(result);
                            CtxResult::Ok(self.ctx.clone())
                        }
                        Err(err) => CtxResult::Err(StrErr(format!(
                            "{}.eval | Read context error: {:?}",
                            self.dbgid, err
                        ))),
                    }
                }
                Err(err) => {
                    CtxResult::Err(StrErr(format!(
                        "{}.eval | Read context error: {:?}",
                        self.dbgid, err
                    )))
                }
            },
        }
    }
}