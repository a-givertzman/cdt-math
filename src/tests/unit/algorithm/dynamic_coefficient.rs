#[cfg(test)]

mod DynamicCoefficient {
    use std::{str::FromStr, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, lifting_speed::lifting_speed::LiftingSpeed, select_bet_phi::select_bet_phi::SelectBetPhi}, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, liftclass::LiftClass, load_combination::LoadCombination}}};
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
    /// Testing [Dk].eval
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                SelectBetPhi::new(LiftClass::Hc1),
                LiftingSpeed::new(DriverType::Hd1, LoadCombination::A1, 0.5, 0.1),
                1.135,
            ),
            (
                01,
                SelectBetPhi::new(LiftClass::Hc2),
                LiftingSpeed::new(DriverType::Hd1, LoadCombination::A1, 0.5, 0.1),
                1.27,
            ),
        ];
        for (step, select_bet_phi, lifting_speed, target) in test_data {
            let result = DynamicCoefficient::new(
                select_bet_phi,
                lifting_speed,
            ).eval();
            match result {
                Ok(result) => {
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Err(err) => panic!("{} | Error: {:?}", dbgid, err),
            }
        }
        test_duration.exit();
    }
}
