#[cfg(test)]

mod param_to_compare {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::hook_chooser::param_comp::Param_to_compare;
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
    fn test_get_fmg() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();

        let test_data =[
            ((0.1,"HC1","A1","HD1",1.0,1.0), 1.19682),
            ((0.1,"HC2","A1","HD2",1.0,1.0), 1.4126400000000003),
            ((0.1,"HC3","A1","HD3",1.0,1.0), 1.6284600000000002),
            ((0.1,"HC4","A1","HD4",1.0,1.0), 1.2556800000000001),
            ((0.1,"HC4","A1","HD5",1.0,1.0), 0.6670800000000001),

            
        ];

        for (value,target) in test_data.into_iter(){
            let result = Param_to_compare::get_fmg(value.0, value.1, value.2, value.3, value.4, value.5);
            println!("{}",result);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
        }

        test_duration.exit();

    }

    #[test]
    fn test_get_din_coeff() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();

        let test_data =[
            ((0.1,1.0,1.0), 1.1),
            ((0.1,5.0,6.0), 30.1),
        ];

        for (value,target) in test_data.into_iter(){
            let result = Param_to_compare::get_din_coeff((value.0,value.1),value.2);
            println!("{}",result);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
        }

        test_duration.exit();

    }

}
