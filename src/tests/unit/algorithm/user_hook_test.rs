#[cfg(test)]

mod UserHook {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::{dbgid::dbgid::DbgId, entities::hook::Hook, user_setup::user_hook::UserHook};
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
                    Hook {
                        gost: "GOST 123".to_string(),
                        r#type: "Type A".to_string(),
                        load_capacity_m13: 25.7,
                        load_capacity_m46: 10.0,
                        load_capacity_m78: 12.0,
                        shank_diameter: 15.0
                    },
                    Hook {
                        gost: "GOST 456".to_string(),
                        r#type: "Type B".to_string(),
                        load_capacity_m13: 30.1,
                        load_capacity_m46: 12.0,
                        load_capacity_m78: 10.0,
                        shank_diameter: 10.0
                    },
                ],
                0,
                Hook {
                    gost: "GOST 123".to_string(),
                    r#type: "Type A".to_string(),
                    load_capacity_m13: 25.7,
                    load_capacity_m46: 10.0,
                    load_capacity_m78: 12.0,
                    shank_diameter: 15.0
                },
            )
        ];
        for (step, filtered_hooks, user_choice, target) in test_data {
            let value = UserHook::new().select(filtered_hooks, user_choice);
            match value {
                Ok(result) => assert_eq!(result, target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
