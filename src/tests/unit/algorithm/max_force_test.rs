#[cfg(test)]
mod max_force {
    use std::{sync::Once, time::Duration};
    use futures::future::BoxFuture;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::{ContextRead, ContextWrite}, ctx_result::CtxResult}, hoisting_tackle_effiency_coefficient::hoist_tackle_eff_coeff_ctx::HoistTackleEffCoeffCtx, initial_ctx::initial_ctx::InitialCtx, load_hand_device_mass::load_hand_device_mass_ctx::LoadHandDeviceMassCtx, maximum_force::{max_force::MaxForce, max_force_ctx::MaxForceCtx}, rope_count::rope_count_ctx::RopeCountCtx}, kernel::{eval::Eval, storage::storage::Storage, types::eval_result::EvalResult}};
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
        let dbg = "max_force";
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
                RopeCountCtx { 
                    result: 5.0, 
                },
                HoistTackleEffCoeffCtx {
                    result: 20.0,
                },
                LoadHandDeviceMassCtx { 
                    total_mass: 50.0,
                    net_weight: 20.0,
                },
                9.81
            ),
            (
                2,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_2",
                ))
                .unwrap(),
                RopeCountCtx { 
                    result: 4.0, 
                },
                HoistTackleEffCoeffCtx {
                    result: 25.0,
                },
                LoadHandDeviceMassCtx { 
                    total_mass: 50.0,
                    net_weight: 20.0,
                },
                5.6898
            ),
            (
                3,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                RopeCountCtx { 
                    result: 5.0, 
                },
                HoistTackleEffCoeffCtx {
                    result: 15.0,
                },
                LoadHandDeviceMassCtx { 
                    total_mass: 16.0,
                    net_weight: 20.0,
                },
                4.7088
            ),
        ];
        for (step,initial,rope_count, hoist_tackle_eff_coeff, load_hand_device_mass,target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(initial),
            };
            ctx.ctx = ctx.ctx.clone().write(rope_count).unwrap();
            ctx.ctx = ctx.ctx.clone().write(hoist_tackle_eff_coeff).unwrap();
            ctx.ctx = ctx.ctx.clone().write(load_hand_device_mass).unwrap();
            let result = MaxForce::new(ctx).eval(()).await;
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<MaxForceCtx>::read(result)
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
                CtxResult::None => panic!("step {} \nerror: `MaxForce` returns None", step),
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
