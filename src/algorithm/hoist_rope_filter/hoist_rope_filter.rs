use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, entities::hoisting_rope::hoisting_rope::HoistingRope, initial_ctx::initial_ctx::InitialCtx, min_break_force::min_break_force_ctx::MinBreakForceCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, types::eval_result::EvalResult}};
use super::hoist_rope_filter_ctx::HoistRopeFilterCtx;
///
/// Calculation step: [filtering hoisting ropes](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
pub struct HoistRopeFilter {
    dbg: DbgId,
    /// value of [filtering hoisting ropes](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    value: Option<HoistRopeFilterCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>
}
//
//
impl HoistRopeFilter {
    ///
    /// New instance [HoistRopeFilter]
    /// - 'ctx' - [Context] instance, where store all info about initial data and each algorithm result's
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbg: DbgId("HoistRopeFilter".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for HoistRopeFilter {
    ///
    /// Method of calculating [filtering hoisting ropes](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            let result = match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx).clone();
                    let min_break_force = ContextRead::<MinBreakForceCtx>::read(&ctx).result.clone();
                    let mut result: Vec<HoistingRope> = Vec::new();
                    for hoisting_rope in initial.hoisting_ropes.iter() {
                        if hoisting_rope.r#type == initial.hoist_rope_type &&
                            initial.hoist_rope_diameters.iter().any(|d| *d == hoisting_rope.rope_diameter) &&
                            hoisting_rope.rope_durability == initial.hoist_rope_durability_class &&
                            hoisting_rope.rope_force >= min_break_force {
                            if result.len() < initial.hoist_rope_count.into() {
                                result.push(hoisting_rope.clone()); 
                            } else { 
                                break; 
                            }
                        }
                    };
                    if result.is_empty() {
                        return CtxResult::Err(StrErr(format!(
                            "{}.eval | Error to find hoisting ropes",
                            self.dbg
                        )))
                    }
                    let result = HoistRopeFilterCtx {
                        result
                    };
                    ctx.write(result)
                }
                CtxResult::Err(err) => CtxResult::Err(StrErr(format!(
                    "{}.eval | Read context error: {:?}",
                    self.dbg, err
                ))),
                CtxResult::None => CtxResult::None,
            };
            result
        })
    }
}
//
//
impl std::fmt::Debug for HoistRopeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoistRopeFilter")
            .field("dbgid", &self.dbg)
            .field("value", &self.value)
            .finish()
    }
}
