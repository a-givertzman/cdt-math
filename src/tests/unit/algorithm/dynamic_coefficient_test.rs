#[cfg(test)]

mod DynamicCoefficient {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, lifting_class::LiftClass, loading_combination::LoadingCombination}}};
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
        let test_data =[
            (
                1,
                LiftClass::Hc1,
                DriverType::Hd1, 
                LoadingCombination::A1, 
                0.1, 
                0.23,
                1.067
            ),
            (
                2,
                LiftClass::Hc2,
                DriverType::Hd2, 
                LoadingCombination::A1, 
                0.33, 
                0.43,
                1.2462
            ),
            (
                3,
                LiftClass::Hc3,
                DriverType::Hd2,
                LoadingCombination::B1, 
                0.12, 
                0.65,
                1.4815
            ),
            (
                4,
                LiftClass::Hc4,
                DriverType::Hd3,
                LoadingCombination::C1,
                0.81, 
                0.95,
                1.4754
            ),
        ];
        for (step, lift_class, driver_type, load_comb, vhmax, vhcs, target) in test_data {
           match DynamicCoefficient::new().eval(lift_class, driver_type, load_comb, vhmax, vhcs) {
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
