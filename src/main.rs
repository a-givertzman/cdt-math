mod algorithm;
mod app;
mod infrostructure;
mod kernel;
#[cfg(test)]
mod tests;
use algorithm::{bearing_filter::bearing_filter_ctx::BearingFilterCtx, context::{context::Context, context_access::ContextRead}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::{bearing::Bearing, hook::Hook}, hook_filter::{hook_filter::HookFilter, hook_filter_ctx::HookFilterCtx}, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use infrostructure::client::{choose_user_bearing::{ChooseUserBearingQuery, ChooseUserBearingReply}, choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query};
use kernel::{eval::Eval, link::Link, mok_user_reply::mok_user_reply::MokUserReply, request::Request, run::Run, storage::storage::Storage, user_setup::{user_bearing::UserBearing, user_hook::UserHook}};
use sal_sync::services::service::service::Service;
///
/// Application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = DbgId("main".into());
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    }
    let cache_path = "./src/tests/unit/kernel/storage/cache/test_2";
    let (local, remote) = Link::split(&dbg);
    let mut mok_user_reply = MokUserReply::new(&dbg, remote);
    let mok_user_reply_handle = mok_user_reply.run().unwrap();
    let _user_hook = UserBearing::new(
        Request::<ChooseUserBearingReply>::new(|ctx: &Context| -> ChooseUserBearingReply {
            let variants: &BearingFilterCtx = ctx.read();
            let query = Query::ChooseUserBearing(ChooseUserBearingQuery::new(variants.result.clone()));
            ctx.link.req(query).expect("{}.req | Error to send request")
        }),
        UserHook::new(
            Request::<ChooseUserHookReply>::new(|ctx: &Context| -> ChooseUserHookReply {
                let variants: &HookFilterCtx = ctx.read();
                let query = Query::ChooseUserHook(ChooseUserHookQuery::test(variants.result.clone()));
                ctx.link.req(query).expect("{}.req | Error to send request")
            }),
            HookFilter::new(
                DynamicCoefficient::new(
                    SelectBettaPhi::new(
                        LiftingSpeed::new(
                            Initial::new(
                                Context::new(
                                    InitialCtx::new(
                                        &mut Storage::new(cache_path)
                                    ).unwrap(),
                                    local.into()
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ).eval();
    mok_user_reply.exit();
    for (_id, h) in mok_user_reply_handle.into_iter() {
        h.join().unwrap();
    }
    Ok(())
}
