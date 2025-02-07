#[cfg(test)]

mod select_bet_phi {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{
        algorithm::{
            context::{context::Context, ctx_result::CtxResult},
            entities::bet_phi::BetPhi,
            initial_ctx::initial_ctx::InitialCtx,
            select_betta_phi::select_betta_phi::SelectBettaPhi,
        },
        kernel::{dbgid::dbgid::DbgId, eval::Eval, storage::storage::Storage, str_err::str_err::StrErr},
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
        let test_data: [(i32, InitialCtx, CtxResult<BetPhi, StrErr>); 4] = [
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_1",
                ))
                .unwrap(),
                CtxResult::Ok(BetPhi {
                    bet: 0.17,
                    phi: 1.05,
                }),
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                CtxResult::Ok(BetPhi {
                    bet: 0.34,
                    phi: 1.1,
                }),
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                CtxResult::Ok(BetPhi {
                    bet: 0.51,
                    phi: 1.15,
                }),
            ),
            (
                4,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_4",
                ))
                .unwrap(),
                CtxResult::Ok(BetPhi {
                    bet: 0.68,
                    phi: 1.2,
                }),
            ),
        ];
        for (step, initial, target) in test_data {
            let ctx = MocEval {
                ctx: Context::new(initial),
            };
            let result = SelectBettaPhi::new(ctx).eval();
            match (&result, &target) {
                (CtxResult::Ok(result), CtxResult::Ok(target)) => {
                    let result = result
                        .select_bet_phi
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
        pub ctx: Context,
    }
    //
    //
    impl Eval for MocEval {
        fn eval(
            &mut self,
        ) -> CtxResult<Context, crate::kernel::str_err::str_err::StrErr> {
            CtxResult::Ok(self.ctx.clone())
        }
    }
}
