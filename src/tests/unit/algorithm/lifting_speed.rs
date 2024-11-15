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
            (LiftingSpeed{
                dbgid: DbgId(format!("LiftingSpeed")),
                driver_type: DriverType::Hd1,
                load_comb: LoadCombination::A1,
                vhmax: 2.0,
                vhcs: 2.0,
                value: None,
            },2.0)
        ];
        for (mut value, target) in test_data.into_iter() {
            let mut result = 0.0;
            match value.eval() {
                Ok(value) => result = value,
                Err(_) => todo!(),
            }
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
        }

        test_duration.exit();
    }
}