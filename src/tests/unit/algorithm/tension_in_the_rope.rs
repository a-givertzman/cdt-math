#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use serde::de::value;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::tension_in_the_rope::tension_in_the_rope::TensionInTheRope;
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
    fn test_task_cycle() {
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
                1,
                (0.0,0.0),
                0.0
            ),
            (
                1,
                (5.0,12.0),
                2.5
            ),
        ];
        for (step,(m_to_lift,hook_weight),target) in test_data.iter(){
            let result = TensionInTheRope::new().eval(*m_to_lift, *hook_weight);
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}