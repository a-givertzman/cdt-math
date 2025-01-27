#[cfg(test)]

mod HoistTackleEfficienyCoeff {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{hoist_tackle_efficiency_coef::hoist_tackle_efficieny_coef::HoistTackleEfficienyCoeff, hoist_tackle_multiplicity::hoist_tackle_multiplicity::HoistTackleMultiplicity}, kernel::{storage::storage::Storage, user_setup::{entities::user_another_load_device, user_load_hand::UserLoadHandDevice}}};
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
        let mut user_select = Storage::new(path);
        let mut storage = Storage::new(path);
        let test_data = [
            (
                1,
                0.9605227500000011
            )
            
        ];
        for (step,target) in test_data.iter() {
            let mut user_load_device: UserLoadHandDevice = UserLoadHandDevice::new();
            let _ = user_load_device.select(&mut user_select, &mut storage);
            match HoistTackleEfficienyCoeff::new().eval(HoistTackleMultiplicity::new(), &mut user_select, user_load_device) {
                Ok(result) => assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
