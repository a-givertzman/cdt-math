#[cfg(test)]
mod SelectBetPhi {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::select_betta_phi::select_betta_phi::SelectBettaPhi, kernel::{dbgid::dbgid::DbgId, entities::{bet_phi::BetPhi, lifting_class::LiftClass}, initial_data::{self, initial_data::InitialData}, storage::storage::Storage}};
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
        let path = "./src/tests/unit/kernel/storage/cache"; 
        let mut storage_initial_data: Storage = Storage::new(path);
        let test_data =[
            (
                1,
                InitialData::new(&mut storage_initial_data),
                BetPhi::new(0.17, 1.05)
            ),
        ];
        for (step,initial_data,target) in test_data {
            let result = SelectBettaPhi::new(initial_data.expect(&format!("{} | step {}, Error of initial data",dbgid, step))).eval();
            assert_eq!(result, target,"step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
