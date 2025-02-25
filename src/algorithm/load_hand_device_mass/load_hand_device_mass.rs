use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{algorithm::{context::{context::Context, context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx}, kernel::{dbgid::dbgid::DbgId, eval::Eval, str_err::str_err::StrErr, user_setup::user_hook_ctx::UserHookCtx}};
use super::load_hand_device_mass_ctx::LoadHandDeviceMassCtx;
///
/// Calculation step: [total mass and net weight](design\docs\algorithm\part02\chapter_02_choose_another_load_handing_device.md)
pub struct LoadHandDeviceMass<'a> {
    dbgid: DbgId,
    /// value of [total mass and net weight](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    value: Option<LoadHandDeviceMassCtx>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<'a, Context> + Send + 'a>,
}
//
//
impl<'a> LoadHandDeviceMass<'a> {
    ///
    /// New instance [LoadHandDeviceMass]
    /// - `ctx` - [Context]
    pub fn new(ctx: impl Eval<'a, Context> + Send + 'a) -> Self {
        Self {
            dbgid: DbgId("LoadHandDeviceMass".to_string()),
            value: None,
            ctx: Box::new(ctx),
        }
    }
}
//
//
#[async_trait]
impl<'a> Eval<'a, Context> for LoadHandDeviceMass<'a> {
    fn eval(&'a mut self) -> BoxFuture<'a, CtxResult<Context, StrErr>> {
        Box::pin(async {
            match self.ctx.eval().await {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let user_hook = ContextRead::<UserHookCtx>::read(&ctx).result.clone();
                    let result = match &initial.user_alt_lift_device {
                        Some(lift_device) => {
                            LoadHandDeviceMassCtx {
                                total_mass: user_hook.weight + lift_device.weight,
                                net_weight: initial.load_capacity - lift_device.weight,
                            }
                        },
                        None => {
                            LoadHandDeviceMassCtx {
                                total_mass: user_hook.weight,
                                net_weight: initial.load_capacity,
                            }
                        },
                    };
                    self.value = Some(result.clone());
                    ctx.write(result)
                },
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
impl std::fmt::Debug for LoadHandDeviceMass<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadHandDeviceMass")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            // .field("ctx", &self.ctx)
            .finish()
    }
}