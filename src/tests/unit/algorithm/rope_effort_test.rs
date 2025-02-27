#[cfg(test)]
mod rope_effort {
    use std::{sync::{mpsc, Once}, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::ContextRead, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx, rope_effort::{rope_effort::RopeEffort, rope_effort_ctx::RopeEffortCtx}}, kernel::{eval::Eval, storage::storage::Storage, sync::switch::Switch, types::eval_result::EvalResult}};
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
    #[tokio::test]
    async fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "eval";
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
                90.0
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                30.0
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                50.0
            )
        ];
        let (send, recv) = mpsc::channel();
        let mut switch = Switch::new(dbg, send, recv);
        for (step,initial,target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(initial),
            };
            let (switch_, result) = RopeEffort::new(ctx).eval(switch).await;
            switch = switch_;
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<RopeEffortCtx>::read(result)
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
                CtxResult::None => panic!("step {} \nerror: `RopeEffort` returns None", step),
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
    impl Eval<Switch, EvalResult> for MocEval {
        fn eval(&'_ mut self, switch: Switch) -> BoxFuture<'_, EvalResult> {
            Box::pin(async {
                (switch, CtxResult::Ok(self.ctx.clone()))
            })
        }
    }
}
