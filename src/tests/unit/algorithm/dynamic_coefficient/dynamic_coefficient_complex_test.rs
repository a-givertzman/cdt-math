#[cfg(test)]

mod dynamic_coefficient {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::{Arc, Once},
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        algorithm::{
            context::{context::Context, context_access::ContextRead, ctx_result::CtxResult},
            dynamic_coefficient::{dynamic_coefficient::DynamicCoefficient, dynamic_coefficient_ctx::DynamicCoefficientCtx},
            initial_ctx::initial_ctx::InitialCtx,
            lifting_speed::lifting_speed::LiftingSpeed,
            select_betta_phi::select_betta_phi::SelectBettaPhi,
        },
        kernel::{dbgid::dbgid::DbgId, eval::Eval, link::Link, storage::storage::Storage, str_err::str_err::StrErr},
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
        let dbg = DbgId("eval".into());
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let (local, _) = Link::split(&dbg);
        let local = Arc::new(local);
        let test_data: [(i32, InitialCtx, CtxResult<f64, StrErr>); 3] = [
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_1",
                ))
                .unwrap(),
                CtxResult::Ok(1.157),
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                CtxResult::Ok(1.168),
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                CtxResult::Ok(1.252),
            ),
        ];
        for (step, initial, target) in test_data {
            let result = DynamicCoefficient::new(
                SelectBettaPhi::new(
                    LiftingSpeed::new(
                        MocEval {
                            ctx: Some(Context::new(
                                initial,
                                local.clone(),
                            )),
                        },
                    ),
                ),
            ).eval();
            match (&result, &target) {
                (CtxResult::Ok(result), CtxResult::Ok(target)) => {
                    let result = ContextRead::<DynamicCoefficientCtx>::read(result)
                        .result;
                    assert!(
                        result == *target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    );
                }
                (CtxResult::Err(_), CtxResult::Err(_)) => {},
                (CtxResult::None, CtxResult::None) => {},
                _ => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            }
        }
        test_duration.exit();
    }
    ///
    ///
    #[derive(Debug)]
    struct MocEval {
        pub ctx: Option<Context>,
    }
    //
    //
    impl Eval for MocEval {
        fn eval(
            &mut self,
        ) -> CtxResult<Context, crate::kernel::str_err::str_err::StrErr> {
            CtxResult::Ok(self.ctx.take().unwrap())
        }
    }
}
