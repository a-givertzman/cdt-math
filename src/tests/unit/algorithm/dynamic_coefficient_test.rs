#[cfg(test)]

mod DynamicCoefficient {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi}, kernel::{dbgid::dbgid::DbgId, entities::{driver_type::DriverType, lifting_class::LiftClass, loading_combination::LoadingCombination}, initial_data::initial_data::InitialData, storage::storage::Storage}};
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
                SelectBettaPhi::new(InitialData::new(&mut storage_initial_data).expect(&format!("{} | Error of initial data",dbgid))),
                LiftingSpeed::new(InitialData::new(&mut storage_initial_data).expect(&format!("{} | Error of initial data",dbgid))),
                1.1571
            ),
        ];
        for (step, select_betta_phi, lifting_speed, target) in test_data {
           let result = DynamicCoefficient::new(select_betta_phi, lifting_speed).eval();
           assert!(result==target,"step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
