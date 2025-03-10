use futures::future::BoxFuture;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, entities::{crane_work_area_type::CraneWorkArea, mechanism_work_type::MechanismWorkType, winding_type::WindingType}, initial_ctx::initial_ctx::InitialCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, types::eval_result::EvalResult}};
use super::safety_factor_ctx::SafetyFactorCtx;
///
/// Calculation step: [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct SafetyFactor {
    dbgid: DbgId,
    /// value of [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<SafetyFactorCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl SafetyFactor {
    ///
    /// New instance [SafetyFactor]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("SafetyFactor".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for SafetyFactor {
    ///
    /// Method of calculating [rope safety factor](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            let result = match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let mechanism_work_type = initial.mechanism_work_type.clone();
                    let winding_type = initial.winding_type.clone();
                    let mark_fire_exp_env = initial.mark_fire_exp_env;
                    let crane_work_area = initial.crane_work_area.clone();
                    let result = SafetyFactorCtx {
                        result: if mark_fire_exp_env {
                            match winding_type {
                                WindingType::SingleLayer => 4.5,
                                WindingType::MultiLayer => 4.5,
                            }
                        } else {
                            match crane_work_area {
                                CraneWorkArea::Default => {
                                    match winding_type {
                                        WindingType::SingleLayer => {
                                            match mechanism_work_type {
                                                MechanismWorkType::M1 => 3.15,
                                                MechanismWorkType::M2 => 3.35,
                                                MechanismWorkType::M3 => 3.55,
                                                MechanismWorkType::M4 => 4.0,
                                                MechanismWorkType::M5 => 4.5,
                                                MechanismWorkType::M6 => 5.6,
                                                MechanismWorkType::M7 => 7.1,
                                                MechanismWorkType::M8 => 9.0,
                                            }
                                        },
                                        WindingType::MultiLayer => {
                                            match mechanism_work_type {
                                                MechanismWorkType::M1 => 3.55,
                                                MechanismWorkType::M2 => 3.55,
                                                MechanismWorkType::M3 => 3.55,
                                                MechanismWorkType::M4 => 4.0,
                                                MechanismWorkType::M5 => 4.5,
                                                MechanismWorkType::M6 => 5.6,
                                                MechanismWorkType::M7 => return CtxResult::Err(StrErr(format!(
                                                    "{}.eval | For multilayer winding for mode `M7` the rope safety factor is unknown",
                                                    self.dbgid
                                                ))),
                                                MechanismWorkType::M8 => return CtxResult::Err(StrErr(format!(
                                                    "{}.eval | For multilayer winding for mode `M8` the rope safety factor is unknown",
                                                    self.dbgid
                                                ))),
                                            }
                                        },
                                    }
                                },
                                _ => {
                                    match winding_type {
                                        WindingType::SingleLayer => 4.5,
                                        WindingType::MultiLayer => 4.5,
                                    } 
                                }
                            }
                        }
                    };
                    ctx.write(result)
                }
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbgid, err
                ))),
                CtxResult::None => CtxResult::None,
            };
            result
        })
    }
}
//
//
impl std::fmt::Debug for SafetyFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SafetyFactor")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}
