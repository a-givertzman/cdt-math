#[cfg(test)]

mod Liftingspeed {
    use std::{sync::{Arc, Once, RwLock}, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{context::{context::Context, ctx_result::CtxResult}, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed}, kernel::{dbgid::dbgid::DbgId, eval::Eval, storage::storage::Storage}};

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
        let path = "./src/tests/unit/kernel/storage/cache"; 
        let mut storage_initial_data: Storage = Storage::new(path);
        test_duration.run().unwrap();
        let test_data =[
            (
                1,
                InitialCtx::new(&mut storage_initial_data).unwrap(),
                CtxResult::Ok(0.63),
            ),
        ];
        for (step,initial, target) in test_data {
            let ctx = Arc::new(RwLock::new(Context::new(initial)));
            let result = LiftingSpeed::new(ctx.clone()).eval();
            let result = result.unwrap().read().unwrap().lifting_speed.result.clone();
            assert!(result == target,"step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
