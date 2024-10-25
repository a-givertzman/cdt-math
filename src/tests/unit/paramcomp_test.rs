#[cfg(test)]

mod din_coeff {
use std::{sync::Once, time::{Duration, Instant}};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::kernel::crane_constructor::hook_chooser::param_comp::ParamComp;
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
fn test_paramcomp() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    init_each();
    log::debug!("");
    let self_id = "test";
    log::debug!("\n{}", self_id);
    let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data =[
        ((1.0,1.0,9.8), 9.8),
        ((10.0,1.0,9.8), 98.0),
        ((10.0,10.0,9.8), 980.0000000000001),

        
    ];
       
    for (value,target) in test_data.into_iter(){
        let result = ParamComp::fmg(value.0, value.1, value.2);
        assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
    }

    
    test_duration.exit();
}
}