#[cfg(test)]

mod request {
    use std::{sync::Once, time::Duration};
    use testing::{entities::test_value::Value, stuff::max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{context::{context::Context, testing_ctx::{MokUserReplyTestCtx, TestingCtx}}, initial_ctx::initial_ctx::InitialCtx}, kernel::{sync::link::Link, request::Request, storage::storage::Storage}};
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
    #[tokio::test]
    async fn execute() {
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
        let (mut local, _) = Link::split(dbg);
        for (step, initial, target) in test_data {
            let value = target.clone();
            let request = Request::<MokUserReplyTestCtx>::new(async |ctx: Context, _link: &mut Link| {
                let reply = ctx
                    .testing.clone()
                    .unwrap()
                    .mok_user_reply;
                reply
            });
            let mut ctx = Context::new(initial.clone());
            ctx.testing = Some(TestingCtx { mok_user_reply: value });
            let result = request.fetch(ctx, &mut local).await;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
