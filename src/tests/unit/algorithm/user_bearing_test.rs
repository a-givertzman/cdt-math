#[cfg(test)]

mod UserBearing {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::{dbgid::dbgid::DbgId, entities::bearing::Bearing, user_setup::user_bearing::UserBearing};
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
    /// Testing to 'select()' method
    #[test]
    fn select() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("select".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
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
                ],
                0,
                Bearing {
                    name: "Bearing1".to_string(),
                    outer_diameter: 50.0,
                    inner_diameter: 20.0,
                    static_load_capacity: 1000.0,
                    height: 15.0,
                },
            )
        ];
        for (step, filtered_bearings, user_choice, target) in test_data {
            let value = UserBearing::new().select(filtered_bearings, user_choice);
            match value {
                Ok(result) => assert_eq!(result, target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
