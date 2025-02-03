#[cfg(test)]

mod DynamicCoefficient {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::{Arc, Once, RwLock},
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        algorithm::{
            context::{context::Context, ctx_result::CtxResult},
            dynamic_coefficient::{
                dynamic_coefficient::DynamicCoefficient,
                dynamic_coefficient_ctx::DynamicCoefficientCtx,
            },
            entities::bet_phi::BetPhi,
            initial_ctx::initial_ctx::InitialCtx,
            lifting_speed::lifting_speed_ctx::LiftingSpeedCtx,
            select_betta_phi::select_betta_phi_ctx::SelectBetPhiCtx,
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
                Context {
                    initial: InitialCtx::new(&mut Storage::new(
                        "./src/tests/unit/kernel/storage/cache/test_1",
                    ))
                    .unwrap(),
                    lifting_speed: LiftingSpeedCtx {
                        result: CtxResult::Ok(50.0),
                    },
                    bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(BetPhi {
                            bet: 0.50,
                            phi: 0.64,
                        }),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(25.64),
            ),
            (
                2,
                Context {
                    initial: InitialCtx::new(&mut Storage::new(
                        "./src/tests/unit/kernel/storage/cache/test_1",
                    ))
                    .unwrap(),
                    lifting_speed: LiftingSpeedCtx {
                        result: CtxResult::Ok(660.0),
                    },
                    bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(BetPhi {
                            bet: 0.120,
                            phi: 0.64,
                        }),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(79.84),
            ),
            (
                3,
                Context {
                    initial: InitialCtx::new(&mut Storage::new(
                        "./src/tests/unit/kernel/storage/cache/test_1",
                    ))
                    .unwrap(),
                    lifting_speed: LiftingSpeedCtx {
                        result: CtxResult::Ok(60.0),
                    },
                    bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(BetPhi {
                            bet: 0.125,
                            phi: 0.24,
                        }),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(7.74),
            ),
        ];
        for (step, contex, target) in test_data {
            let ctx = Arc::new(RwLock::new(contex));
            let result = DynamicCoefficient::new(ctx.clone()).eval();
            let result = result
                .unwrap()
                .read()
                .unwrap()
                .dynamic_coefficient
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
        test_duration.exit();
    }
}
