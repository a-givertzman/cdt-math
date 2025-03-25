#[cfg(test)]

mod user_hoist_rope {
    use std::{sync::Once, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        algorithm::{
            context::{context::Context, context_access::ContextRead, ctx_result::CtxResult}, entities::hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType}, hoist_rope_filter::hoist_rope_filter_ctx::HoistRopeFilterCtx, initial_ctx::initial_ctx::InitialCtx
        },
        infrostructure::client::{choose_hoisting_rope::ChooseHoistingRopeQuery, query::Query},
        kernel::{eval::Eval, mok_user_reply::mok_user_reply::MokUserReply, request::Request, storage::storage::Storage, sync::{link::Link, switch::Switch}, types::eval_result::EvalResult, user_setup::{user_hoist_rope::UserHoistRope, user_hoist_rope_ctx::UserHoistRopeCtx}}
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
        let dbg = "user_hoist_rope";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
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
        for (step, target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap()),
            };
            let result = UserHoistRope::new(
                Request::new(
                    switch.link().await,
                    async |variants: HoistRopeFilterCtx, link: Link| {
                        let query = Query::ChooseHoistingRope(ChooseHoistingRopeQuery::new(variants.result.clone()));
                        (link.req(query).await.expect("{}.req | Error to send request"), link)
                    },
                ),
                ctx
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
    ///
    ///
    #[derive(Debug, Clone)]
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
