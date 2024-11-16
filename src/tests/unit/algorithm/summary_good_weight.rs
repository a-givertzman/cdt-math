#[cfg(test)]

mod SummaryGoodWeights {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::hook_choose::summary_good_weights::summary_good_weight::SummaryGoodWeights;
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
                (0.0,0.0)
            ),
            (
                1,
                (1.0,2.0,3.0),
                (1.0,3.0)
            )
        ];
        for (step,value,target) in test_data.iter(){
            let mut result = SummaryGoodWeights::new();
            result.eval(value.0, value.1, value.2);
            assert!(result.good_weight == target.0, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            assert!(result.summary_weight == target.1, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
