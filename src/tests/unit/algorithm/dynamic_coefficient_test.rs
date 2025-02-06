#[cfg(test)]

mod dynamic_coefficient {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::{Arc, Once, RwLock},
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        algorithm::{
            context::{context::Context, ctx_result::CtxResult}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi
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
                )).unwrap(),
                CtxResult::Ok(1.1571),
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                )).unwrap(),
                CtxResult::Ok(1.1680000000000001),
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                )).unwrap(),
                CtxResult::Ok(1.252),
            ),
        ];
        for (step, initial, target) in test_data {
            let ctx = MocEval { ctx: Context::new(initial) };
            let result = DynamicCoefficient::new(
                SelectBettaPhi::new(
                    LiftingSpeed::new(ctx)))
            .eval();
            let result = result.unwrap().read().unwrap().dynamic_coefficient.result.clone();
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
        pub ctx: Context
    }
    //
    //
    impl Eval for MocEval {
        fn eval(&mut self) -> CtxResult<Arc<RwLock<Context>>, crate::kernel::str_err::str_err::StrErr> {
            CtxResult::Ok(Arc::new(RwLock::new(self.ctx.clone())))
        }
    }
}
