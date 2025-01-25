#[cfg(test)]

mod HoistTackleMultiplicity {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{hoist_tackle_multiplicity::hoist_tackle_multiplicity::HoistTackleMultiplicity, hoisting_tackle::hoisting_tackle::HoistingTackle, ropes_count::ropes_count::RopesCount}, kernel::{storage::storage::Storage, user_setup::user_load_hand::UserLoadHandDevice}};
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
    fn test_task_cycle() {
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
        let mut storage = Storage::new(path);
        let test_data = [
            (
                1,
                2.0
            )
        ];
        for (step,target) in test_data.iter() {
            let mut user_load_device = UserLoadHandDevice::new();
            let _ = user_load_device.select(&mut user_select, &mut storage);
            match HoistTackleMultiplicity::new().eval(&mut RopesCount::new(), &mut HoistingTackle::new(), &mut user_select, &mut user_load_device){
                Ok(result) => {
                    assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                },
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
