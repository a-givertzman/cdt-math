#[cfg(test)]

mod user_hoist {
    use std::{
        sync::Once, 
        time::Duration
    };
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{
        DebugSession, 
        LogLevel, 
        Backtrace
    };
    use crate::{
        algorithm::{
            context::{
                context::Context, 
                context_access::ContextRead, 
                ctx_result::CtxResult
            }, entities::hoist::{hoist::Hoist, hoist_driver_type::HoistDriverType, hoist_duty_group::HoistDutyGroup, hoist_headroom::HoistHeadroomType, hoist_manufacturer_type::HoistManufacturerType, hoist_mobility_type::HoistMobilityType, hoist_perfomance_type::HoistPerfomanceType, hoist_type::HoistType, hoist_wheel_count::HoistWheelCount}, hoist_filter::hoist_filter_ctx::HoistFilterCtx, hook_filter::hook_filter_ctx::HookBlockFilterCtx, initial_ctx::initial_ctx::InitialCtx
        },
        infrostructure::client::{
            choose_user_hoist::ChooseUserHoistQuery, choose_user_hook::ChooseUserHookQuery, query::Query
        },
        kernel::{
            eval::Eval, mok_user_reply::mok_user_reply::MokUserReply, request::Request, storage::storage::Storage, sync::{link::Link, switch::Switch}, types::eval_result::EvalResult, user_setup::{user_hoist::{user_hoist::UserHoist, user_hoist_ctx::UserHoistCtx}, user_hook::{user_hook::UserHookBlock, user_hook_ctx::UserHookCtx}} 
        }
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
    /// Testing such functionality / behavior
    #[tokio::test(flavor = "multi_thread")]
    async fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "user_hoist";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                Hoist { 
                    manufacturer: HoistManufacturerType::Nante, 
                    name: "HoistTest_1".to_string(), 
                    hoist_type: HoistType::Rope, 
                    driver_type: HoistDriverType::Electric, 
                    hoist_mobility: HoistMobilityType::Stationary, 
                    hoist_perfomance: HoistPerfomanceType::Industrial, 
                    hoist_headroom: HoistHeadroomType::Deacreased, 
                    load: 50.0, 
                    lifting_height: 100.0, 
                    duty_group: HoistDutyGroup::M1, 
                    wheel_count: HoistWheelCount::Eight, 
                    lifting_speed: 20.0, 
                    travelling_speed: 10.0, 
                },
            )
        ];
        let (switch, remote) = Switch::split(dbg);
        let switch_handle = switch.run().await.unwrap();
        let mut mok_user_reply = MokUserReply::new(dbg, remote);
        let mok_user_reply_handle = mok_user_reply.run().await.unwrap();
        let initial = InitialCtx::new(&mut Storage::new(
            "./src/tests/unit/kernel/storage/cache/test_1",
        )).unwrap();
        for (step, target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(initial.clone()),
            };
            let result = UserHoist::new(
                Request::new(
                    switch.link().await,
                    async |variants: HoistFilterCtx, link: Link| {
                        let query = Query::ChooseUserHoist(ChooseUserHoistQuery::test(variants.result.clone()));
                        (link.req(query).await.expect("{}.req | Error to send request"), link)
                    },
                ),
                ctx
            )
            .eval(())
            .await;
            match result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<UserHoistCtx>::read(&result)
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
                CtxResult::None => panic!("step {} \nerror: `UserHoist` returns None", step),
            }
        }
        switch.exit();
        mok_user_reply.exit();
        switch_handle.join_all().await;
        mok_user_reply_handle.await.unwrap();
        test_duration.exit();
    }
    ///
    ///
    #[derive(Debug)]
    struct MocEval {
        pub ctx: Context,
    }
    //
    //
    impl Eval<(), EvalResult> for MocEval {
        fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
            Box::pin(async {
                CtxResult::Ok(self.ctx.clone())
            })
        }
    }
}
