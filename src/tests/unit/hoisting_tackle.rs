#[cfg(test)]

mod hoisting_tackle {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::hoisting_tackle::hoisting_tackle;
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
    fn test_cablecount() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data_1 =[
            ((20.0,"20.0".to_string(),"20.0".to_string()), 2.0),
            ((10.0,"10.0".to_string(),"10.0".to_string()), 2.0),
            ((11.0,"11.0".to_string(),"11.0".to_string()), 2.0),
            
        ];
        
        for (value,target) in test_data_1.into_iter(){
            let result = hoisting_tackle::hoisting_tackle::cable_count(value.0, &value.1, &value.2);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
        }

        
        test_duration.exit();
    }
}
