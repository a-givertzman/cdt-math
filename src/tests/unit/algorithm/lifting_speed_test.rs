#[cfg(test)]

mod lifting_speed {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::{Arc, Once, RwLock},
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        algorithm::{
            context::{context::Context, ctx_result::CtxResult},
            initial_ctx::initial_ctx::InitialCtx,
            lifting_speed::lifting_speed::LiftingSpeed,
        },
        kernel::{dbgid::dbgid::DbgId, eval::Eval, storage::storage::Storage},
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
    fn init_each() {}
    ///
    /// Testing to 'eval()' method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("eval".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_1",
                ))
                .unwrap(),
                CtxResult::Ok(0.63),
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                CtxResult::Ok(0.2),
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                CtxResult::Ok(0.2),
            ),
            (
                4,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_4",
                ))
                .unwrap(),
                CtxResult::Ok(0.315),
            ),
            (
                5,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_5",
                ))
                .unwrap(),
                CtxResult::Ok(0.0),
            ),
            (
                6,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_6",
                ))
                .unwrap(),
                CtxResult::Ok(0.63),
            ),
            (
                7,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_7",
                ))
                .unwrap(),
                CtxResult::Ok(0.2),
            ),
            (
                8,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_8",
                ))
                .unwrap(),
                CtxResult::Ok(0.315),
            ),
        ];
        for (step, initial, target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(initial),
            };
            let result = LiftingSpeed::new(ctx).eval();
            let result = result.unwrap().read().unwrap().lifting_speed.result.clone();
            assert!(
                result == target,
                "step {} \nresult: {:?}\ntarget: {:?}",
                step,
                result,
                target
            );
        }
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
    impl Eval for MocEval {
        fn eval(
            &mut self,
        ) -> CtxResult<Arc<RwLock<Context>>, crate::kernel::str_err::str_err::StrErr> {
            CtxResult::Ok(Arc::new(RwLock::new(self.ctx.clone())))
        }
    }
}
