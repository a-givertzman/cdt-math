#[cfg(test)]

mod LiftingSpeed {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::lifting_speed::lifting_speed::LiftingSpeed, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, loading_combination::LoadingCombination}}};
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
    /// Testing to 'eval()' method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("eval".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let mut lifting_speed = LiftingSpeed::new();
        let test_data =[
            (
                1,
                DriverType::Hd1, 
                LoadingCombination::A1, 
                0.1, 
                0.23,
                0.1
            ),
            (
                2,
                DriverType::Hd2, 
                LoadingCombination::A1, 
                0.33, 
                0.43,
                0.43
            ),
            (
                3,
                DriverType::Hd2,
                LoadingCombination::B1, 
                0.12, 
                0.65,
                0.65
            ),
            (
                4,
                DriverType::Hd3,
                LoadingCombination::C1,
                0.81, 
                0.95,
                0.405
            ),
        ];
        for (step, driver_type, load_comb, vhmax, vhcs, target) in test_data {
            let result = lifting_speed.eval(driver_type, load_comb, vhmax, vhcs);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);     
        }
        test_duration.exit();
    }
}
