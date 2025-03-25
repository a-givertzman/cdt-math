#[cfg(test)]
mod hoist_rope_filter {
    use std::{sync::Once, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::ContextRead, ctx_result::CtxResult}, entities::hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType}, hoist_rope_filter::{hoist_rope_filter::HoistRopeFilter, hoist_rope_filter_ctx::HoistRopeFilterCtx}, initial_ctx::initial_ctx::InitialCtx}, kernel::{eval::Eval, storage::storage::Storage, types::eval_result::EvalResult}};
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
        let dbg = "hoist_rope_filter";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_1",
                ))
                .unwrap(),
                vec![
                    HoistingRope { 
                        name: "Rope A".to_string(),
                        rope_diameter: 10.5,
                        r#type: RopeType::Metal,
                        rope_durability: RopeDurabilityClass::C1370,
                        rope_force: 1500.0,
                        s: 78.5,
                        m: 0.85
                    }
                ]
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                vec![
                    HoistingRope { 
                        name: "Rope A".to_string(),
                        rope_diameter: 10.5,
                        r#type: RopeType::Metal,
                        rope_durability: RopeDurabilityClass::C1370,
                        rope_force: 1500.0,
                        s: 78.5,
                        m: 0.85
                    },
                    HoistingRope { 
                        name: "Rope C".to_string(), 
                        rope_diameter: 12.0, 
                        r#type: RopeType::Metal, 
                        rope_durability: RopeDurabilityClass::C1370, 
                        rope_force: 800.0, 
                        s: 50.3, 
                        m: 0.9 }
                ]
            ),

        ];
        for (step,initial,target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(initial),
            };
            let result = HoistRopeFilter::new(ctx).eval(()).await;
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<HoistRopeFilterCtx>::read(result)
                        .result.clone();
                    assert!(
                        result == target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    )
                }
                CtxResult::Err(err) => panic!("step {} \nerror: {:#?}", step, err),
                CtxResult::None => panic!("step {} \nerror: `HoistRopeFilter` returns None", step),
            }
        }
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
