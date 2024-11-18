#[cfg(test)]

mod HookFilter {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{algorithm::{hook_choose::filter::hook_filter::HookFilter, storage::storage::Storage}, kernel::entities::mechanism_work_type::MechanismWorkType};
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
    fn filter() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();

        // Заполнение хранилища
        let mut storage = Storage::new();
        if let Err(e) = storage.load_from_json("C:\\Users\\klaim\\Desktop\\cdt-math\\src\\kernel\\storage\\storage.json") {
            eprintln!("Error loading JSON: {}", e);
            return;
        }

        let test_data = [
            (
                ("крюк однорогий", MechanismWorkType::M1, 0.5),
                3
            )
        ];

        for ((hook_type, mechanism_work_type, m_to_lift), target) in test_data.into_iter() {
            let mut hook_filter = HookFilter::new();
            let result = hook_filter.filter(&mut storage, hook_type.to_string(), mechanism_work_type, m_to_lift);
            assert!(result.len() == target, "result: {:?}\ntarget: {:?}", result.len(), target)
        }
        test_duration.exit(); 
    }
}
