mod algorithm;
mod app;
mod infrostructure;
mod kernel;
#[cfg(test)]
mod tests;
use algorithm::{
    bearing_filter::bearing_filter_ctx::BearingFilterCtx, context::{context::Context, context_access::ContextRead}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, hoisting_tackle::hoisting_tackle::HoistingTackle, hook_filter::{hook_filter::HookFilter,hook_filter_ctx::HookFilterCtx}, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, load_hand_device_mass::load_hand_device_mass::LoadHandDeviceMass, rope_count::rope_count::RopeCount, rope_effort::rope_effort::RopeEffort, select_betta_phi::select_betta_phi::SelectBettaPhi
};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use infrostructure::client::{
    change_hoisting_tackle::{ChangeHoistingTackleQuery, ChangeHoistingTackleReply}, choose_user_bearing::{ChooseUserBearingQuery, ChooseUserBearingReply}, choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query
};
use kernel::{
    eval::Eval, mok_user_reply::mok_user_reply::MokUserReply, request::Request, run::Run, storage::storage::Storage, sync::{link::Link, switch::Switch}, user_setup::{user_bearing::UserBearing, user_hook::UserHook}
};
use tokio::sync::mpsc;
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
    let (send, recv) = mpsc::channel(10_000);
    let mut switch = Switch::new(&dbg, send, recv);
    let switch_handle = switch.run().unwrap();
    let mut mok_user_reply = MokUserReply::new(&dbg, switch.link());
    let mok_user_reply_handle = mok_user_reply.run().await.unwrap();
    let _test = HoistingTackle::new(
        switch.link(),
        Request::<ChangeHoistingTackleReply>::new(|ctx: Context, link: &mut Link| {
            let query = Query::ChangeHoistingTackle(ChangeHoistingTackleQuery::new());
            link.req(query).await.expect("{}.req | Error to send request")
        }),
        RopeCount::new(
            RopeEffort::new(
                LoadHandDeviceMass::new(
                    UserBearing::new(
                        switch.link(),
                        Request::<ChooseUserBearingReply>::new(|ctx: Context, link: &mut Link| {
                            let variants: &BearingFilterCtx = ctx.read();
                            let query = Query::ChooseUserBearing(ChooseUserBearingQuery::new(variants.result.clone()));
                            link.req(query).await.expect("{}.req | Error to send request")
                        }),
                        UserHook::new(
                            switch.link(),
                            Request::<ChooseUserHookReply>::new(|ctx: Context, link: &mut Link| async move {
                                let variants: &HookFilterCtx = ctx.read();
                                let query = Query::ChooseUserHook(ChooseUserHookQuery::test(variants.result.clone()));
                                link.req(query).await.expect("{}.req | Error to send request")
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
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
    ).eval()
    .await;
    mok_user_reply.exit();
    switch.exit();
    switch_handle.join_all().await;
    mok_user_reply_handle.join_all().await;
    Ok(())
}
