#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::polispast_type::polispast_type::PolispastType;
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
                (0.0,0.0)
                ,1
            ),
            (
                2,
                (2.0,5.0)
                ,1
            ),
            (
                3,
                (2.0,112.0)
                ,2
            ),
        ];
        for (step,value,target) in test_data.iter(){
            let result = PolispastType::new().eval(value.0, value.1);
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);

        }
        test_duration.exit();
    }
}
