use crate::{
    algorithm::{context::{context::Context, ctx_result::CtxResult}, entities::{driver_type::DriverType, hook::Hook, loading_combination::LoadingCombination, mechanism_work_type::MechanismWorkType}},
    kernel::{
        dbgid::dbgid::DbgId,
        eval::Eval,
        str_err::str_err::StrErr,
    },
};
use std::sync::{Arc, RwLock};
///
/// Сlass, that filter hooks by user loading capacity
/// [reference to filtering documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
#[derive(Debug, Clone)]
pub struct HookFilter {
    dbgid: DbgId,
    /// vector of [filtered hooks](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<Vec<Hook>>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Arc<RwLock<Context>>,
}
//
//
impl HookFilter {
    ///
    /// Class Constructor
    /// - `ctx` - [Context]
    pub fn new(ctx: Arc<RwLock<Context>>) -> Self {
        Self {
            dbgid: DbgId("HookFilter".to_string()),
            value: None,
            ctx,
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
        match self.value {
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
                let user_loading_capacity = initial.load_capacity.clone();
                let user_mech_work_type = initial.mechanism_work_type.clone();
                let hooks = initial.hooks.clone();
                let mut result: Vec<Hook> = Vec::new();
                for hook in hooks.iter() {
                    match user_mech_work_type {
                        MechanismWorkType::M1 | MechanismWorkType::M2 | MechanismWorkType::M3  => {
                            if hook.load_capacity_m13 >= user_loading_capacity { result.push(hook.clone()) }
                        },
                        MechanismWorkType::M4 | MechanismWorkType::M5 | MechanismWorkType::M6 => {
                            if hook.load_capacity_m46 >= user_loading_capacity { result.push(hook.clone()) }
                        },
                        MechanismWorkType::M7 | MechanismWorkType::M8 => {
                            if hook.load_capacity_m78 >= user_loading_capacity { result.push(hook.clone()) }
                        }
                    }
                }
                self.value = Some(result.clone());
                match self.ctx.write() {
                    Ok(mut ctx) => {
                        if result.is_empty() {
                            ctx.filtered_hooks.result = CtxResult::None;
                            return CtxResult::Ok(self.ctx.clone())
                        }
                        ctx.filtered_hooks.result = CtxResult::Ok(result.clone());
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
