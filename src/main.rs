mod algorithm;
mod app;
mod infrostructure;
mod kernel;
#[cfg(test)]
mod tests;
use algorithm::{
    bearing_filter::bearing_filter_ctx::BearingFilterCtx, context::context::Context, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, hoist_rope_filter::{hoist_rope_filter::HoistRopeFilter, hoist_rope_filter_ctx::HoistRopeFilterCtx}, hoisting_tackle::hoisting_tackle::HoistingTackle, hoisting_tackle_effiency_coefficient::hoist_tackle_eff_coeff::HoistTackleEffCoeff, hoisting_tackle_multiplicity::hoist_tackle_multi::HoistTackleMulti, hook_filter::{hook_filter::HookFilter,hook_filter_ctx::HookFilterCtx}, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, load_hand_device_mass::load_hand_device_mass::LoadHandDeviceMass, maximum_force::max_force::MaxForce, min_break_force::min_break_force::MinBreakForce, rope_count::rope_count::RopeCount, rope_effort::rope_effort::RopeEffort, rope_safety_factor::safety_factor::SafetyFactor, select_betta_phi::select_betta_phi::SelectBettaPhi
};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use infrostructure::client::{
    change_hoisting_tackle::ChangeHoistingTackleQuery, choose_hoisting_rope::ChooseHoistingRopeQuery, choose_user_bearing::ChooseUserBearingQuery, choose_user_hook::ChooseUserHookQuery, query::Query
};
use kernel::{
    eval::Eval, mok_user_reply::mok_user_reply::MokUserReply, request::Request, run::Run,
    storage::storage::Storage, sync::{link::Link, switch::Switch},
    user_setup::{user_bearing::UserBearing, user_hoist_rope::UserHoistRope, user_hook::UserHook},
};
///
/// Application entry point
// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[tokio::main]
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = DbgId("main".into());
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    }
    let cache_path = "./src/tests/unit/kernel/storage/cache/test_2";
    let (switch, remote) = Switch::split(&dbg);
    let switch_handle = switch.run().await.unwrap();
    let mut mok_user_reply = MokUserReply::new(&dbg, remote);
    let mok_user_reply_handle = mok_user_reply.run().await.unwrap();
    let _result = UserHoistRope::new(
        Request::new(
            switch.link().await,
            async |variants: HoistRopeFilterCtx, link: Link| {
                let query = Query::ChooseHoistingRope(ChooseHoistingRopeQuery::new(variants.result.clone()));
                (link.req(query).await.expect("{}.req | Error to send request"), link)
            },
        ),
        HoistRopeFilter::new(
            MinBreakForce::new(
                SafetyFactor::new(
                    MaxForce::new(
                        HoistTackleEffCoeff::new(
                            HoistTackleMulti::new(
                                HoistingTackle::new(
                                    Request::new(
                                        switch.link().await,
                                        async |_: (), link: Link| {
                                            let query = Query::ChangeHoistingTackle(ChangeHoistingTackleQuery::new());
                                            (link.req(query).await.expect("{}.req | Error to send request"), link)
                                        }
                                    ),
                                    RopeCount::new(
                                        RopeEffort::new(
                                            LoadHandDeviceMass::new(
                                                UserBearing::new(
                                                    Request::new(
                                                        switch.link().await,
                                                        async |variants: BearingFilterCtx, link: Link| {
                                                            let query = Query::ChooseUserBearing(ChooseUserBearingQuery::new(variants.result.clone()));
                                                            (link.req(query).await.expect("{}.req | Error to send request"), link)
                                                        },
                                                    ),
                                                    UserHook::new(
                                                        Request::new(
                                                            switch.link().await,
                                                            async |variants: HookFilterCtx, link: Link| {
                                                                let query = Query::ChooseUserHook(ChooseUserHookQuery::test(variants.result.clone()));
                                                                (link.req(query).await.expect("{}.req | Error to send request"), link)
                                                            },
                                                        ),
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
                                    ),
                                )
                            )
                        )
                    )
                )
            )
        )
    )  
    .eval(())
    .await;
    switch.exit();
    mok_user_reply.exit();
    switch_handle.join_all().await;
    mok_user_reply_handle.await.unwrap();
    Ok(())
}
