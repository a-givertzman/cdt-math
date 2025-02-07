#[cfg(test)]

mod dynamic_coefficient {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        algorithm::{
            context::{context::Context, ctx_result::CtxResult}, dynamic_coefficient::{dynamic_coefficient::DynamicCoefficient, dynamic_coefficient_ctx::DynamicCoefficientCtx}, entities::bet_phi::BetPhi, hook_filter::hook_filter_ctx::HookFilterCtx, initial_ctx::initial_ctx::InitialCtx, lifting_speed::{lifting_speed::LiftingSpeed, lifting_speed_ctx::LiftingSpeedCtx}, select_betta_phi::{select_betta_phi::SelectBettaPhi, select_betta_phi_ctx::SelectBetPhiCtx}
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
                    select_bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(
                            BetPhi {
                                bet: 5.0,
                                phi: 15.0,
                            }
                        ),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                    hook_filter: HookFilterCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(265.0),
            ),
            (
                2,
                Context {
                    initial: InitialCtx::new(&mut Storage::new(
                        "./src/tests/unit/kernel/storage/cache/test_1",
                    ))
                    .unwrap(),
                    lifting_speed: LiftingSpeedCtx {
                        result: CtxResult::Ok(50.0),
                    },
                    select_bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(
                            BetPhi {
                                bet: 52.0,
                                phi: 16.0,
                            }
                        ),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                    hook_filter: HookFilterCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(2616.0),
            ),
            (
                3,
                Context {
                    initial: InitialCtx::new(&mut Storage::new(
                        "./src/tests/unit/kernel/storage/cache/test_1",
                    ))
                    .unwrap(),
                    lifting_speed: LiftingSpeedCtx {
                        result: CtxResult::Ok(50.0),
                    },
                    select_bet_phi: SelectBetPhiCtx {
                        result: CtxResult::Ok(
                            BetPhi {
                                bet: 35.0,
                                phi: 25.0,
                            }
                        ),
                    },
                    dynamic_coefficient: DynamicCoefficientCtx {
                        result: CtxResult::None,
                    },
                    hook_filter: HookFilterCtx {
                        result: CtxResult::None,
                    },
                },
                CtxResult::Ok(1775.0),
            ),
        ];
        for (step, ctx, target) in test_data {
            let ctx = MocEval {
                ctx: ctx,
            };
            let result =
                DynamicCoefficient::new(ctx).eval();
            let result = result
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
