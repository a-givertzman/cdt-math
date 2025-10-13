#[cfg(test)]

mod usage_class {
    use debugging::session::debug_session::{
        Backtrace, 
        DebugSession,
        LogLevel
    };
    use futures::future::BoxFuture;
    use sal_sync::services::entity::error::str_err::StrErr;
    use std::{
        sync::Once,
        time::Duration,
    };
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{
        algorithm::{
            choice_usage_class::{usage_class::ChoiceUsageClass, usage_class_ctx::ChoiceUsageClassCtx}, context::{
                context::Context, 
                context_access::ContextRead, 
                ctx_result::CtxResult
            }, entities::{hoist_group::HoistGroup, usage_class::UsageClass}, initial_ctx::initial_ctx::InitialCtx
        },
        kernel::{
            dbgid::dbgid::DbgId, eval::Eval, storage::storage::Storage, types::eval_result::EvalResult
        },
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
    #[tokio::test(flavor = "multi_thread")]
    async fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId("usage_class".into());
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data: [(i32, HoistGroup, CtxResult<ChoiceUsageClassCtx, StrErr>); 4] = [
            (
                1,
                HoistGroup::M1,
                CtxResult::Ok(ChoiceUsageClassCtx { 
                    result: UsageClass::T0
                }),
            ),
            (
                2,
                HoistGroup::M2,
                CtxResult::Ok(ChoiceUsageClassCtx { 
                    result: UsageClass::T1
                }),
            ),
            (
                3,
                HoistGroup::M3,
                CtxResult::Ok(ChoiceUsageClassCtx { 
                    result: UsageClass::T2
                }),
            ),
            (
                4,
                HoistGroup::M4,
                CtxResult::Ok(ChoiceUsageClassCtx { 
                    result: UsageClass::T3
                }),
            ),
        ];
        let mut initial = InitialCtx::new(&mut Storage::new(
            "./src/tests/unit/kernel/storage/cache/test_1",
        )).unwrap();
        for (step, hoist_group, target) in test_data {
            initial.hoist_group = hoist_group;
            let ctx = MocEval {
                ctx: Context::new(initial.clone()),
            };
            let result = ChoiceUsageClass::new(ctx).eval(()).await;
            match (&result, &target) {
                (CtxResult::Ok(result), CtxResult::Ok(target)) => {
                    let result = ContextRead::<ChoiceUsageClassCtx>::read(result)
                        .result.clone();
                    assert!(
                        result == target.result,
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
    impl Eval<(), EvalResult> for MocEval {
        fn eval(&mut self, _: ()) -> BoxFuture<'_, EvalResult> {
            Box::pin(async {
                CtxResult::Ok(self.ctx.clone())
            })
        }
    }
}
