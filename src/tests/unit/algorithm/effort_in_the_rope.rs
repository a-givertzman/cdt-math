#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::effort_in_the_rope::effort_in_the_rope::EffortInTheRope;
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
                1,
                0.0,
                7.5
            ),
            (
                1,
                120.0,
                130.0
            ),
        ];
        for (step,value,target) in test_data.iter(){
            let result = EffortInTheRope::new().eval(*value);
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);

        }
        test_duration.exit();
    }
}