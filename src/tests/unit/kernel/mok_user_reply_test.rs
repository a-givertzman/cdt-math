#[cfg(test)]

mod mok_user_reply {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::{link::Link, mok_user_reply::{mok_user_reply::MokUserReply, query_struct::QueryStruct}};
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
    /// Testing 'run' method
    #[test]
    fn run() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                QueryStruct::new(),
            )
        ];
        let (local, remote) = Link::split("TestUser");
        let user = MokUserReply::new(remote);
        let _handle = user.run().unwrap();
        for (step, target) in test_data.iter() {
            let result: QueryStruct = local.req(target).expect("Request failed");
            assert!(result.data == target.data, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    
}
