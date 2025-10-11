use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use super::hook_filter_ctx::HookBlockFilterCtx;
use crate::{
    algorithm::{
        context::{
            context_access::{
            ContextRead, 
            ContextWrite
        }, 
        ctx_result::CtxResult
        },
        entities::{
            hook::HookBlock, 
            hoist_group::HoistGroup
        }, 
        initial_ctx::initial_ctx::InitialCtx,
    },
    kernel::{
        dbgid::dbgid::DbgId, 
        eval::Eval, 
        types::eval_result::EvalResult
    },
};
///
/// Calculation step: [filtering hook blocks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct HookFilter {
    dbgid: DbgId,
    /// vector of [filtered hook block](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<HookBlockFilterCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl  HookFilter {
    ///
    /// New instance [HookFilter]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self {
            dbgid: DbgId("HookFilter".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for HookFilter {
    ///
    /// Method of filtering hook blocks by user loading capacity
    /// [reference to filtering documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    match self.value.clone() {
                        Some(hook_filter) => ctx.write(hook_filter),
                        None => {
                            let initial = ContextRead::<InitialCtx>::read(&ctx);
                            let user_loading_capacity = initial.load.clone();
                            let user_mech_work_type = initial.hoist_group.clone();
                            let result: Vec<HookBlock> = initial
                                .hooks
                                .iter()
                                .cloned()
                                .filter(|hook| match user_mech_work_type {
                                    HoistGroup::M1
                                    | HoistGroup::M2
                                    | HoistGroup::M3 => {
                                        hook.load_m13 >= user_loading_capacity
                                    }
                                    HoistGroup::M4
                                    | HoistGroup::M5
                                    | HoistGroup::M6 => {
                                        hook.load_m13 >= user_loading_capacity
                                    }
                                    HoistGroup::M7 | HoistGroup::M8 => {
                                        hook.load_m13 >= user_loading_capacity
                                    }
                                    HoistGroup::M9 => {
                                        false
                                    }
                                })
                                .collect();
                            if result.is_empty() {
                                CtxResult::Err(StrErr(format!(
                                    "{}.eval | No available variants of hook for specified requirements",
                                    self.dbgid,
                                )))
                            } else {
                                let result = HookBlockFilterCtx { result };
                                self.value = Some(result.clone());
                                ctx.write(result)
                            }
                        }
                    }
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
impl std::fmt::Debug for HookFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HookFilter")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}
