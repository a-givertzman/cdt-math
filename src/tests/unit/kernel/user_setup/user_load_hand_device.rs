#[cfg(test)]

mod UserLoadHandDevice {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::{dbgid::dbgid::DbgId, entities::{another_load_device::AnLoadDevice, hook::Hook}, storage::storage::Storage, user_setup::{entities::{user_another_load_device::UserAnLoadDevice, user_hook::UserHook}, user_load_hand::UserLoadHandDevice}};
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
        let path = "./src/tests/unit/kernel/storage/cache";
        let mut user_select = Storage::new(path);
        let mut storage = Storage::new(path);
        let test_data: [(i32, (Hook, Option<AnLoadDevice>)); 1] = [
            (
                1,
                (
                    Hook {
                        gost: "GOST 456".to_string(),
                        r#type: "Type B".to_string(),
                        load_capacity_m13: 30.1,
                        load_capacity_m46: 12.0,
                        load_capacity_m78: 10.0,
                        shank_diameter: 10.0,
                        weight: 50.0
                    },
                    None
                ),
            )
        ];
        for (step,target) in test_data {
            match UserLoadHandDevice::new().select(&mut user_select, &mut storage) {
                Ok((user_hook,user_an_load_deivce)) => {
                    match user_hook.hook {
                        Some(hook) => {
                            assert_eq!(hook, target.0,"step {} \nresult: {:?}\ntarget: {:?}", step, hook, target);
                            assert_eq!(user_an_load_deivce.an_device, target.1,"step {} \nresult: {:?}\ntarget: {:?}", step, hook, target);
                        },
                        None => panic!("{} | step {},  Error", dbgid, step)
                    }
                }
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err)
            }
        }
        test_duration.exit();
    }
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
        let mut user_select = Storage::new(path);
        let mut storage = Storage::new(path);
        let test_data = [
            (
                1,
                (50.0,100.0)
            )
        ];
        for (step, target) in test_data {
            let mut value = UserLoadHandDevice::new();
            let _ = value.select(&mut user_select, &mut storage);
            match value.weights(&mut user_select) {
                Ok(result) => assert_eq!(result, target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
        }
        test_duration.exit();
    }
}
