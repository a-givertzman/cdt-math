#[cfg(test)]

mod HoistingTackle {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{hoisting_tackle::hoisting_tackle::HoistingTackle, rope_effort::rope_effort::RopeEffort, ropes_count::ropes_count::RopesCount}, kernel::{storage::storage::Storage, user_setup::user_load_hand::UserLoadHandDevice}};
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
    /// Testing 'eval' method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = "eval";
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = "./src/tests/unit/kernel/storage/cache";
        let user_select = Storage::new(path);
        let mut storage = Storage::new(path);
        let test_data = [ 
            (
                1,
                RopesCount::new(),
                RopeEffort::new(),
                user_select,
                UserLoadHandDevice::new(),
                1.0
            )
        ];
        for (step,mut ropes_count, rope_effort,mut user_select,mut user_load_device,target) in test_data {
            let _ = user_load_device.select(&mut user_select, &mut storage);
            match HoistingTackle::new().eval(&mut ropes_count, &mut user_select, rope_effort, &mut user_load_device) {
                Ok(result) => assert_eq!(result,target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
            
        }
        test_duration.exit();
    }
}