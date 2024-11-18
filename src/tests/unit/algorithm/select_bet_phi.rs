#[cfg(test)]

mod user_select {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        result, sync::Once, time::{Duration, Instant}
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{algorithm::select_bet_phi::select_bet_phi::{BetPhi, SelectBetPhi}, kernel::{dbgid::dbgid::DbgId, entities::liftclass::LiftClass}};
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
                LiftClass::Hc1,
                BetPhi{ bet: 0.17, phi: 1.05}
            ),
        ];
        for (lift_class, target) in test_data.into_iter() {
            match SelectBetPhi::new().eval(lift_class) {
                Ok(result) => assert!(result == target, "result: {:?}\ntarget: {:?}", result, target),
                Err(_) => todo!(),
            } 
        }

        test_duration.exit();
    }
}