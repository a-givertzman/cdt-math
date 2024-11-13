#[cfg(test)]

mod hoisting_tackle {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::{hoist_rope::hoist_rope::HoistRope, hook_chooser::hook::Hook};
    use crate::kernel::storage::storage::Storage;
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
    fn test_s_get() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();





        let test_data =[
            ((2.0, 1.0,9.81,2.0,2.0),7.3575),
            ((2.0, 6.0,9.81,2.0,6.0),6.54),
            ((2.0, 1.0,9.81,4.0,2.0),3.67875),
            ((2.0, 1.0,9.81,2.0,25.0),0.5886),

            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistRope::s_get(value.0, value.1, value.2, value.3, value.4);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }

    #[test]
    fn test_n_get() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();





        let test_data =[
            ((2.0, 2.0),0.9605227500000011),
            ((2.0, 6.0),0.9041751722818402),
            ((2.0, 34.0),0.5921981297876855),
            ((2.0, 25.0),0.6784861035141121),

            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistRope::n_get(value.0, value.1);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }
}
