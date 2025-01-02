#[cfg(test)]
mod SelectBetPhi {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::select_betta_phi::select_betta_phi::SelectBettaPhi, kernel::{dbgid::dbgid::DbgId, entities::{bet_phi::BetPhi, lifting_class::LiftClass}}};
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
    fn init_each() {}
    ///
    /// Testing to 'eval()' method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("eval".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data =[
            (
                1,
                LiftClass::Hc1,
                BetPhi::new(0.17, 1.05)
            ),
            (
                2,
                LiftClass::Hc2,
                BetPhi::new(0.34, 1.10)
            ),
            (
                3,
                LiftClass::Hc3,
                BetPhi::new(0.51, 1.15)
            ),
            (
                4,
                LiftClass::Hc4,
                BetPhi::new(0.68, 1.20)
            ),
        ];
        for (step,lift_class,target) in test_data {
            match SelectBettaPhi::new().eval(lift_class) {
                Ok(result) => assert!(result.bet == target.bet && result.phi == target.phi, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
