#[cfg(test)]

mod RopeReserveRatio {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::rope_reserve_ratio::rope_reserve_ratio::RopeReserveRatio, kernel::entities::mechanism_work_type::{self, MechanismWorkType}};
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
        let test_data=[
            (
                1,
                MechanismWorkType::M1,
                3.15,
            ),
        ];
        for (step,mechanism_work_type,target) in test_data.iter(){
            let result = RopeReserveRatio::new().eval(mechanism_work_type.clone());
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
