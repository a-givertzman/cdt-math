#[cfg(test)]
mod min_break_force {
    use std::{sync::Once, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx, load_hand_device_mass::load_hand_device_mass_ctx::LoadHandDeviceMassCtx, maximum_force::{max_force::MaxForce, max_force_ctx::MaxForceCtx}, min_break_force::{min_break_force::MinBreakForce, min_break_force_ctx::MinBreakForceCtx}, rope_count::{rope_count::RopeCount, rope_count_ctx::RopeCountCtx}, rope_effort::rope_effort_ctx::RopeEffortCtx, rope_safety_factor::safety_factor_ctx::SafetyFactorCtx}, kernel::{eval::Eval, storage::storage::Storage, types::eval_result::EvalResult}};
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
        let dbg = "min_break_force";
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
                MaxForceCtx { 
                    result: 20.0 
                },
                SafetyFactorCtx { 
                    result: 20.0 
                },
                400.0
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                MaxForceCtx { 
                    result: -20.0 
                },
                SafetyFactorCtx { 
                    result: 20.0 
                },
                -400.0
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                MaxForceCtx { 
                    result: -20.5 
                },
                SafetyFactorCtx { 
                    result: 10.0 
                },
                -205.0
            )
        ];
        for (step,initial,max_force,safety_factor,target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(initial),
            };
            ctx.ctx = ctx.ctx.clone().write(max_force).unwrap();
            ctx.ctx = ctx.ctx.clone().write(safety_factor).unwrap();
            let result = MinBreakForce::new(ctx).eval(()).await;
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<MinBreakForceCtx>::read(result)
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
                CtxResult::None => panic!("step {} \nerror: `MinBreakForce` returns None", step),
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
