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
    ctx: Box<dyn Eval<(), EvalResult> + Send>
}
//
//
impl SafetyFactor {
    ///
    /// Value of safety factor for M1-M8 type of [MechanismWorkType] and single [WindingType].
    const SINGLE_COEFF: [f64; 8] = [3.15, 3.35, 3.55, 4.0, 4.5, 5.6, 7.1, 9.0];
    ///
    /// Value of safety factor for M1-M8 type of [MechanismWorkType] and multi [WindingType].
    const MULTI_COEFF: [f64; 8] = [3.55, 3.55, 3.55, 4.0, 4.5, 5.6, 0.0, 0.0]; // M7 and M8 are invalid
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
    ///
    /// Method to select [SafetyFactor] coefficient from [table choice](references\GOST_33710-2015.pdf)
    pub fn select_coeff(
        &mut self,
        winding_type: WindingType,
        mark_fire_exp_env: bool,
        crane_work_area: CraneWorkArea,
        mechanism_work_type: MechanismWorkType,
    ) -> Result<SafetyFactorCtx, StrErr> {
        let result = if mark_fire_exp_env {
            match winding_type {
                WindingType::SingleLayer => Self::SINGLE_COEFF[4],
                WindingType::MultiLayer => Self::SINGLE_COEFF[4],
            }
        } else {
            match crane_work_area {
                CraneWorkArea::Default => {
                    match winding_type {
                        WindingType::SingleLayer => {
                            match mechanism_work_type {
                                MechanismWorkType::M1 => Self::SINGLE_COEFF[0],
                                MechanismWorkType::M2 => Self::SINGLE_COEFF[1],
                                MechanismWorkType::M3 => Self::SINGLE_COEFF[2],
                                MechanismWorkType::M4 => Self::SINGLE_COEFF[3],
                                MechanismWorkType::M5 => Self::SINGLE_COEFF[4],
                                MechanismWorkType::M6 => Self::SINGLE_COEFF[5],
                                MechanismWorkType::M7 => Self::SINGLE_COEFF[6],
                                MechanismWorkType::M8 => Self::SINGLE_COEFF[7],
                            }
                        }
                        WindingType::MultiLayer => {
                            match mechanism_work_type {
                                MechanismWorkType::M1 => Self::MULTI_COEFF[0],
                                MechanismWorkType::M2 => Self::MULTI_COEFF[1],
                                MechanismWorkType::M3 => Self::MULTI_COEFF[2],
                                MechanismWorkType::M4 => Self::MULTI_COEFF[3],
                                MechanismWorkType::M5 => Self::MULTI_COEFF[4],
                                MechanismWorkType::M6 => Self::MULTI_COEFF[5],
                                MechanismWorkType::M7 => {
                                    return Err(StrErr(format!(
                                        "{}.eval | For multilayer winding for mode `M7` the rope safety factor is unknown",
                                        self.dbgid
                                    )))
                                }
                                MechanismWorkType::M8 => {
                                    return Err(StrErr(format!(
                                        "{}.eval | For multilayer winding for mode `M8` the rope safety factor is unknown",
                                        self.dbgid
                                    )))
                                }
                            }
                        }
                    }
                }
                _ => {
                    match winding_type {
                        WindingType::SingleLayer => Self::SINGLE_COEFF[4],
                        WindingType::MultiLayer => Self::MULTI_COEFF[4],
                    }
                }
            }
        };
        Ok(SafetyFactorCtx { result })
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
                    let result = match self.select_coeff(winding_type, mark_fire_exp_env, crane_work_area, mechanism_work_type) {
                        Ok(coeff) => coeff,
                        Err(err) => return CtxResult::Err(StrErr(format!(
                            "{}.eval | Select safety factor coefficient error: {:?}",
                            self.dbgid, err
                        ))),
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
