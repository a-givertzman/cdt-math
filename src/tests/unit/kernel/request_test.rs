#[cfg(test)]

mod request {
    use std::{sync::Once, time::Duration};
    use testing::{entities::test_value::Value, stuff::max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{context::{context::Context, testing_ctx::MokUserReplyTestCtx}, initial_ctx::initial_ctx::InitialCtx}, kernel::{request::Request, storage::storage::Storage}};
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
    /// Testing 'Request::fetch'
    #[test]
    fn execute() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "fetch";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data: [(usize, InitialCtx, MokUserReplyTestCtx); 2] = [
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                MokUserReplyTestCtx { value: Value::String("Hello World!".to_string()) },
            ),
            (
                1,
                InitialCtx::new(&mut Storage::new(
                    "./src/tests/unit/kernel/storage/cache/test_3",
                ))
                .unwrap(),
                MokUserReplyTestCtx {value: Value::Real(123.456) },
            )
        ];
        for (step, initial, target) in test_data {
            let request = Request::new(|ctx: Context| -> MokUserReplyTestCtx {
                let reply = ctx.testing.unwrap().mok_user_reply;
                reply
            });
            let ctx = Context::new(initial.clone());
            let result = request.fetch(ctx);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
