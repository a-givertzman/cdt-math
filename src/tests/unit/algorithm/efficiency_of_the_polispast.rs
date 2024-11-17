#[cfg(test)]

mod EfficiencyOfThePolyspast {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::efficiency_of_the_polyspast::efficiency_of_the_polyspast::EfficiencyOfThePolyspast;
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
        let test_data =[
            (
                1,
                (0.0,0.0,0.0),
                0.9900000000000011
            ),
            (
                1,
                (2.0,0.5,13.4),
                0.9605227500000011
            ),
        ];
        for (step,(rejecting_blocks,m_to_lift,hook_weight),target) in test_data.iter(){
            let result = EfficiencyOfThePolyspast::new(*rejecting_blocks).eval(*m_to_lift, *hook_weight);
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);

        }
        test_duration.exit();
    }
}
