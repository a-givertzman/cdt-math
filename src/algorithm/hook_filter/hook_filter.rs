use crate::{
    algorithm::{
        context::{context::Context, ctx_result::CtxResult},
        entities::{hook::Hook, mechanism_work_type::MechanismWorkType},
    },
    kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr},
};
use std::sync::{Arc, RwLock};
use super::hook_filter_ctx::HookFilterCtx;
///
/// Сlass, that filter hooks by user loading capacity
/// [reference to filtering documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct HookFilter {
    dbgid: DbgId,
    /// vector of [filtered hooks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<HookFilterCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval>,
}
//
//
impl HookFilter {
    ///
    /// Class Constructor
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval + 'static) -> Self {
        Self {
            dbgid: DbgId("HookFilter".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval for HookFilter {
    ///
    /// Method of filtering hooks by user loading capacity
    /// [reference to filtering documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, StrErr> {
        match self.ctx.eval() {
            CtxResult::Ok(ctx) => {
                let result = match self.value.clone() {
                    Some(hook_filter) => hook_filter,
                    None => {
                        let initial = match ctx.read() {
                            Ok(ctx) => ctx.initial.clone(),
                            Err(err) => {
                                return CtxResult::Err(StrErr(format!(
                                    "{}.eval | Read context error: {:?}",
                                    self.dbgid, err
                                )))
                            }
                        };
                        let user_loading_capacity = initial.load_capacity.clone();
                        let user_mech_work_type = initial.mechanism_work_type.clone();
                        let hooks = initial.hooks.clone();
                        let mut result: Vec<Hook> = Vec::new();
                        for hook in hooks.iter() {
                            match user_mech_work_type {
                                MechanismWorkType::M1 | MechanismWorkType::M2 | MechanismWorkType::M3 => {
                                    if hook.load_capacity_m13 >= user_loading_capacity {
                                        result.push(hook.clone())
                                    }
                                }
                                MechanismWorkType::M4 | MechanismWorkType::M5 | MechanismWorkType::M6 => {
                                    if hook.load_capacity_m46 >= user_loading_capacity {
                                        result.push(hook.clone())
                                    }
                                }
                                MechanismWorkType::M7 | MechanismWorkType::M8 => {
                                    if hook.load_capacity_m78 >= user_loading_capacity {
                                        result.push(hook.clone())
                                    }
                                }
                            }
                        }
                        let result = if result.is_empty() {
                            CtxResult::Err(StrErr(format!(
                                "{}.eval | No available variants of hook for specified requirements",
                                self.dbgid,
                            )))
                        } else {
                            CtxResult::Ok(result)
                        };
                        HookFilterCtx {result}
                    }
                };
                self.value = Some(result.clone());
                ctx.write().unwrap().hook_filter = result;
                CtxResult::Ok(ctx)
            }
            CtxResult::Err(err) => CtxResult::Err(StrErr(format!("{}.eval | Read context error: {:?}", self.dbgid, err))),
            CtxResult::None => CtxResult::None,
        }
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