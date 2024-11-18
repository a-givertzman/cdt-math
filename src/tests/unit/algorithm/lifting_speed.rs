#[cfg(test)]

mod user_select {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{algorithm::lifting_speed::lifting_speed::LiftingSpeed, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, load_combination::LoadCombination}}};
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
    /// Testing such functionality / behavior
    #[test]
    fn eval() {
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
                (DriverType::Hd1,LoadCombination::A1,20.0,20.0),
                20.0,
            ),
            (
                (DriverType::Hd2,LoadCombination::A1,25.0,20.0),
                20.0,
            ),
        ];
        for ((driver_type, load_comb, vhmax, vhcs), target) in test_data.iter() {
            let mut result = 0.0;
            match LiftingSpeed::new().eval(driver_type.clone(), load_comb.clone(), *vhmax, *vhcs){
                Ok(value) => result=value,
                Err(_) => todo!(),
            }
            assert!(result == *target, "result: {:?}\ntarget: {:?}", result, target);
        }

        test_duration.exit();
    }
}