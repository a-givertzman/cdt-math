#[cfg(test)]

mod ForceOfGravity {
    use std::{env::var, str::FromStr, sync::Once, time::{Duration, Instant}};
    use env_logger::Target;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::force_of_gravity::force_of_gravity::ForceOfGravity, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, lift_class::LiftClass, load_combination::LoadCombination}}};
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
                1,
                (0.0,LiftClass::Hc2,DriverType::Hd2,LoadCombination::A1,20.0,20.0),
                0.0
            ),
            (
                1,
                (0.5,LiftClass::Hc1,DriverType::Hd1,LoadCombination::A1,20.0,20.0),
                21.827250000000003
            )
        ];
        for (step,(m_to_lift, lift_class, driver_type, load_comb, vhmax, vhcs),target) in test_data {
            match ForceOfGravity::new().eval(m_to_lift, lift_class, driver_type, load_comb, vhmax, vhcs){
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => {},
            }
        }
        test_duration.exit();
    }
}
