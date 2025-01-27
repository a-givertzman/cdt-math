#[cfg(test)]

mod RopeSafetyFactor {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::rope_safety_factor::rope_safety_factor::RopeSafetyFactor, kernel::storage::storage::Storage};
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
    /// Testing 'select' method
    #[test]
    fn select() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = "eval";
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = "./src/tests/unit/kernel/storage/cache";
        let mut user_select = Storage::new(path);
        let test_data = [
            (
                1,
                3.15
            )
        ];
        for (step,target) in test_data.iter() {
            match RopeSafetyFactor::new().select(&mut user_select) {
                Ok(result) => assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(_) => todo!(),
            }
        }
        test_duration.exit();
    }
}
