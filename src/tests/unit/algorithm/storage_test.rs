#[cfg(test)]
mod tests {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::algorithm::storage::storage::Storage;
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
    fn test_get() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let hooks_storage = Storage::new("src\\kernel\\storage\\construction.hooks.json");
        let test_data = [
            (1,
            "type.one-horned.sequence_number.1.capacity_M1",
            0.4),
        ];
        for (step,path,target) in test_data{
            match hooks_storage.load(path) {
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => log::error!("{}.load | Some Error: {:#?}",hooks_storage.dbgid, err),
            }
        }
        test_duration.exit();
    }
}
