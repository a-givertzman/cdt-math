#[cfg(test)]

mod user_hoist_rope_complex {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        algorithm::{
            bearing_filter::bearing_filter_ctx::BearingFilterCtx, context::{context::Context, context_access::ContextRead, ctx_result::CtxResult}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType}, hoist_rope_filter::{hoist_rope_filter::HoistRopeFilter, hoist_rope_filter_ctx::HoistRopeFilterCtx}, hoisting_tackle::hoisting_tackle::HoistingTackle, hoisting_tackle_effiency_coefficient::hoist_tackle_eff_coeff::HoistTackleEffCoeff, hoisting_tackle_multiplicity::hoist_tackle_multi::HoistTackleMulti, hook_filter::{hook_filter::HookFilter, hook_filter_ctx::HookFilterCtx}, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, load_hand_device_mass::load_hand_device_mass::LoadHandDeviceMass, maximum_force::max_force::MaxForce, min_break_force::min_break_force::MinBreakForce, rope_count::rope_count::RopeCount, rope_effort::rope_effort::RopeEffort, rope_safety_factor::safety_factor::SafetyFactor, select_betta_phi::select_betta_phi::SelectBettaPhi
        },
        infrostructure::client::{change_hoisting_tackle::ChangeHoistingTackleQuery, choose_hoisting_rope::ChooseHoistingRopeQuery, choose_user_bearing::ChooseUserBearingQuery, choose_user_hook::ChooseUserHookQuery, query::Query},
        kernel::{eval::Eval, mok_user_reply::mok_user_reply::MokUserReply, request::Request, storage::storage::Storage, sync::{link::Link, switch::Switch}, user_setup::{user_bearing::UserBearing, user_hoist_rope::UserHoistRope, user_hoist_rope_ctx::UserHoistRopeCtx, user_hook::UserHook}}
    };
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing 'eval'
    #[tokio::test(flavor = "multi_thread")]
    async fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "user_hoist_rope_complex";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                r#"./src/tests/unit/kernel/storage/cache/test_2"#,
                HoistingRope {
                    name: "STO 71915393-TU 051-2014 Octopus 826K".to_owned(),
                    rope_diameter: 12.0,
                    r#type: RopeType::Metal,
                    rope_durability: RopeDurabilityClass::C1770,
                    rope_force: 112.0,
                    s: 67.824,
                    m: 0.688,
                },
            )
        ];
        let (switch, remote) = Switch::split(dbg);
        let switch_handle = switch.run().await.unwrap();
        let mut mok_user_reply = MokUserReply::new(dbg, remote);
        let mok_user_reply_handle = mok_user_reply.run().await.unwrap();
        for (step, cache_path, target) in test_data {
            let result = UserHoistRope::new(
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
            match result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<UserHoistRopeCtx>::read(&result)
                        .result
                        .clone();
                    assert!(
                        result == target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    );
                }
                CtxResult::Err(err) => panic!("step {} \nerror: {:#?}", step, err),
                CtxResult::None => panic!("step {} \nerror: `UserHoistRope` returns None", step),
            }
        }
        switch.exit();
        mok_user_reply.exit();
        mok_user_reply_handle.await.unwrap();
        switch_handle.join_all().await;
        test_duration.exit();
    }
}
