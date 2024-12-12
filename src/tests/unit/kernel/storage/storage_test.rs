#[cfg(test)]
mod tests {
    use std::{sync::Once, time::Duration};
    use api_tools::debug::dbg_id::DbgId;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::storage::storage::Storage;
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
    /// Testing such functionality / behavior
    #[test]
    fn test_load() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = "./src/tests/unit/kernel/storage/storage_test.rs";
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (1,
            "type.one-horned.sequence_number.1.capacity_M2",
            0.2),
        ];
        for (step,path,target) in test_data{
            match hooks_storage.load(path) {
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => panic!("{}.load | Some Error: {:#?}", dbgid, err),
            }
        }
        test_duration.exit();
    }
}
