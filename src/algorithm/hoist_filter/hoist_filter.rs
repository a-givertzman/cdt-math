use futures::future::BoxFuture;
use sal_sync::services::entity::error::str_err::StrErr;
use crate::{
    algorithm::{context::{context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, entities::hoist::hoist::Hoist, hoist_filter::hoist_filter_ctx::HoistFilterCtx, initial_ctx::initial_ctx::InitialCtx}, infrostructure::client::choose_user_hoist_characteristics::ChooseHoistCharactetisticsReply, kernel::{dbgid::dbgid::DbgId, eval::Eval, request::Request, types::eval_result::EvalResult, user_setup::{user_hoist::user_hoist_ctx::UserHoistCtx, user_hook::user_hook_ctx::UserHookCtx}}
};
///
/// Represents user hoist and make request to user for choosing one
pub struct HoistFilter {
    dbgid: DbgId,
    /// value of user hook
    value: Option<UserHoistCtx>,
    /// Event interface
    req: Request<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), ChooseHoistCharactetisticsReply>,
    /// [Context] instance, where store all info about initial data and each algorithm result's
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl HoistFilter {
    ///
    /// New instance [HoistFilter]
    /// - `ctx` - [Context]
    /// - `req` - [Request] for user
    pub fn new(req: Request<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), ChooseHoistCharactetisticsReply>, ctx: impl Eval<(), EvalResult> + Send + 'static) -> Self {
        Self { 
            dbgid: DbgId("HoistFilter".to_string()), 
            value: None,
            req,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for HoistFilter {
    fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
        Box::pin(async {
            let result = self.ctx.eval(()).await;
            match result {
                CtxResult::Ok(ctx) => {
                    let initial = ContextRead::<InitialCtx>::read(&ctx);
                    let hoists = initial.hoists.clone();
                    let user_hook = ContextRead::<UserHookCtx>::read(&ctx).result.clone();
                    let reply = self.req.fetch(
                        (
                            vec!["Rope".to_string(), "Chain".to_string(), "Any".to_string()],
                            vec!["Four".to_string(), "Eight".to_string(), "Any".to_string()],
                            vec!["Normal".to_string(), "Deacreased".to_string(), "Any".to_string()],
                            vec!["Nante".to_string(), "BalkanskoEcho".to_string(),"Stahl".to_string()]
                        )
                    ).await;
                    let mut result: Vec<Hoist> = Vec::new();
                    for hoist in hoists {
                        if (user_hook.clone().hoist_load(initial.hoist_group.clone()) <= hoist.load) 
                        && (initial.lifting_height <= hoist.lifting_height) 
                        && (initial.vhmax_lift_hoist <= hoist.lifting_speed)
                        && (initial.vhmax_travel_hoist <= hoist.travelling_speed)
                        && ((initial.hoist_group.to_string().chars().last().unwrap() as usize) <= (hoist.duty_group.to_string().chars().last().unwrap() as usize)) 
                        && (initial.explosion_fire_safe_crane_purpose.to_string() == hoist.hoist_perfomance.to_string()) 
                        && (reply.hoist_count_wheel == hoist.wheel_count) 
                        && (reply.headroom_type == hoist.hoist_headroom)
                        && (reply.hoist_manufacturer == hoist.manufacturer) {
                            match reply.hoist_type {
                                Ok(ref hoist_type) => {
                                    if hoist.hoist_type == *hoist_type {
                                        result.push(hoist);
                                    }
                                },
                                Err(ref _any) => {
                                    result.push(hoist);
                                    continue;
                                },
                            }
                        }
                    }
                    ctx.write(
                        HoistFilterCtx { 
                            result: result 
                        }
                    )
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
impl std::fmt::Debug for HoistFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoistFilter")
            .field("dbgid", &self.dbgid)
            .field("value", &self.value)
            .finish()
    }
}
