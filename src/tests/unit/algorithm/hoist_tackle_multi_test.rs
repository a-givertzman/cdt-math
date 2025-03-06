#[cfg(test)]
mod hoist_tackle_multi {
    use std::{sync::Once, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoisting_tackle::hoisting_tackle_ctx::HoistingTackleCtx, hoisting_tackle_multiplicity::{hoist_tackle_multi::HoistTackleMulti, hoist_tackle_multi_ctx::HoistTackleMultiCtx}, initial_ctx::initial_ctx::InitialCtx, rope_count::rope_count_ctx::RopeCountCtx}, kernel::{eval::Eval, storage::storage::Storage, types::eval_result::EvalResult}};
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
        let dbg = "hoist_tackle_multi";
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
                HoistingTackleCtx { 
                    result: 20 
                },
                RopeCountCtx {
                    result: 50.0,
                },
                2.5
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                HoistingTackleCtx { 
                    result: 25 
                },
                RopeCountCtx {
                    result: 50.0,
                },
                2.0
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                HoistingTackleCtx { 
                    result: 50 
                },
                RopeCountCtx {
                    result: 50.0,
                },
                1.0
            ),
        ];
        for (step,initial,hoist_tackle,rope_count,target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(initial),
            };
            ctx.ctx = ctx.ctx.clone().write(hoist_tackle).unwrap();
            ctx.ctx = ctx.ctx.clone().write(rope_count).unwrap();
            let result = HoistTackleMulti::new(ctx).eval(()).await;
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<HoistTackleMultiCtx>::read(result)
                        .result;
                    assert!(
                        result == target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    )
                }
                CtxResult::Err(err) => panic!("step {} \nerror: {:#?}", step, err),
                CtxResult::None => panic!("step {} \nerror: `HoistTackleMulti` returns None", step),
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
