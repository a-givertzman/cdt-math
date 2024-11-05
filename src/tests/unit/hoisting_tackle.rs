#[cfg(test)]

mod hoisting_tackle {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::{hoisting_tackle::hoisting_tackle::HoistingTackle, hook_chooser::hook::Hook};
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
    fn test_a_select() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();





        let test_data =[
            (2.0, 1),
            (1.0, 1),
            (0.5, 1),
            (5.0, 2),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistingTackle::a_select(value);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }

    #[test]
    fn test_cable_count() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();





        let test_data =[
            ((7.5,  1.0,1.0),2.0),
            ((10.0, 100.0,2.0),12.0),
            ((20.0, 3.0,1.0),2.0),
            ((30.0, 2.0,5.0),2.0),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistingTackle::cable_count(value.0, value.1, value.2);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }

    #[test]
    fn test_round_to_nearest() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();





        let test_data =[
            (7.5,8.0),
            (11.0,12.0),
            (15.0,16.0),
            (14.0,16.0),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistingTackle::round_to_nearest(value);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }

    #[test]
    fn test_s_select() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();

        let test_data =[
            (1.0 ,7.5),
            (2.6 ,20.0),
            (6.0 ,20.0),
            (10.0 , 30.0),
            (15.0 , 40.0),
            (20.0 , 50.0),
            (40.0 , 60.0),
            (100.0 ,90.0),
            (150.0 ,130.0),
            (200.0 ,180.0),
            (500.0 ,220.0),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = HoistingTackle::s_select(value);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);        
        }
        
        test_duration.exit();
    }

}
