#[cfg(test)]
mod BearingFilter {
    use std::{sync::Once, time::Duration};
    use api_tools::debug::dbg_id::DbgId;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::bearing_filter::bearing_filter::BearingFilter, kernel::{entities::{bearing::Bearing, hook::Hook}, storage::storage::Storage}};
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
    /// Testing filter() method
    #[test]
    fn filter() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("filter".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = "./src/tests/unit/kernel/storage/cache";
        let mut storage = Storage::new(path);
        let mut user_select = Storage::new(path);
        let test_data = [
            (
                1,
                Hook {
                    gost: "GOST 123".to_string(),
                    r#type: "Type A".to_string(),
                    load_capacity_m13: 25.7,
                    load_capacity_m46: 10.0,
                    load_capacity_m78: 12.0,
                    shank_diameter: 61.0
                },
                0.63,
                vec![
                    Bearing {
                        name: "Bearing1".to_string(),
                        outer_diameter: 50.0,
                        inner_diameter: 20.0,
                        static_load_capacity: 1000.0,
                        height: 15.0,
                    },
                    Bearing {
                        name: "Bearing2".to_string(),
                        outer_diameter: 60.0,
                        inner_diameter: 30.0,
                        static_load_capacity: 1500.0,
                        height: 20.0,
                    },
                    Bearing {
                        name: "Bearing3".to_string(),
                        outer_diameter: 40.0,
                        inner_diameter: 15.0,
                        static_load_capacity: 800.0,
                        height: 10.0,
                    }
                ]
            )
        ];
        for (step,user_hook,dynamic_coefficient,target) in test_data {
            match BearingFilter::new().filter(&mut storage, &mut user_select, user_hook, dynamic_coefficient) {
                Ok(ref result) => assert_eq!(*result,target,"step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
