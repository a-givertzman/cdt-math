#[cfg(test)]

mod hook {
    use crate::kernel::crane_constructor::hook_chooser::param_comp::Param_to_compare;
    use crate::kernel::crane_constructor::hook_chooser::{all_hooks::AllHooks, hook::Hook};
    use crate::kernel::storage::storage::Storage;
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;
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
    fn test_bearingcheck() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();

        // Заполнение хранилища пока только крюки
        let mut storage = Storage::new();

        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/",
            Ok(0.4),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/",
            Ok(0.5),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/грузоподъемность/M1/",
            Ok(0.63),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/грузоподъемность/M1/",
            Ok(0.8),
        );

        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/",
            Ok(53.0),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/",
            Ok(47.0),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/диаметр хвостовика/",
            Ok(41.0),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/диаметр хвостовика/",
            Ok(34.0),
        );

        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/масса заготовки/",
            Ok(0.2),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/масса заготовки/",
            Ok(0.25),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/масса заготовки/",
            Ok(0.4),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/масса заготовки/",
            Ok(0.6),
        );

        storage.set(
            "конструкции/подшипники/название/8100Н/статическая грузоподъемность/",
            Ok(11800.0),
        );
        storage.set(
            "конструкции/подшипники/название/8101Н/статическая грузоподъемность/",
            Ok(12900.0),
        );
        storage.set(
            "конструкции/подшипники/название/8100Н/наружный диаметр/",
            Ok(24.0),
        );
        storage.set(
            "конструкции/подшипники/название/8101Н/наружный диаметр/",
            Ok(26.0),
        );

        storage.set(
            "конструкции/подшипники/название/8102Н/статическая грузоподъемность/",
            Ok(14000.0),
        );
        storage.set(
            "конструкции/подшипники/название/8103Н/статическая грузоподъемность/",
            Ok(16600.0),
        );
        storage.set(
            "конструкции/подшипники/название/8102Н/наружный диаметр/",
            Ok(28.0),
        );
        storage.set(
            "конструкции/подшипники/название/8103Н/наружный диаметр/",
            Ok(30.0),
        );

        storage.set(
            "конструкции/подшипники/название/8104Н/статическая грузоподъемность/",
            Ok(22400.0),
        );
        storage.set(
            "конструкции/подшипники/название/8105Н/статическая грузоподъемность/",
            Ok(30000.0),
        );
        storage.set(
            "конструкции/подшипники/название/8104Н/наружный диаметр/",
            Ok(35.0),
        );
        storage.set(
            "конструкции/подшипники/название/8105Н/наружный диаметр/",
            Ok(42.0),
        );

        storage.set(
            "конструкции/подшипники/название/8106Н/статическая грузоподъемность/",
            Ok(33500.0),
        );
        storage.set(
            "конструкции/подшипники/название/8107Н/статическая грузоподъемность/",
            Ok(39000.0),
        );
        storage.set(
            "конструкции/подшипники/название/8106Н/наружный диаметр/",
            Ok(47.0),
        );
        storage.set(
            "конструкции/подшипники/название/8107Н/наружный диаметр/",
            Ok(52.0),
        );

        let test_data = [
            (
                (0.4, "крюк однорогий", "M1", "1"),
                vec![
                    "8100Н", "8101Н", "8102Н", "8103Н", "8104Н", "8105Н", "8106Н", "8107Н",
                ],
            ),
            (
                (0.5, "крюк однорогий", "M1", "2"),
                vec![
                    "8100Н", "8101Н", "8102Н", "8103Н", "8104Н", "8105Н", "8106Н",
                ],
            ),
            (
                (0.63, "крюк однорогий", "M1", "3"),
                vec!["8100Н", "8101Н", "8102Н", "8103Н", "8104Н"],
            ),
            (
                (0.8, "крюк однорогий", "M1", "4"),
                vec!["8100Н", "8101Н", "8102Н", "8103Н"],
            ),
        ];

        for (value, target) in test_data.into_iter() {
            let result = Hook::bearing_check(
                Param_to_compare::get_fmg(3.0, "M1", "A1", "HD1", 1.0, 1.0),
                &value.1.to_string(),
                &mut storage,
                &vec![value.3.to_string()],
            );
            assert!(
                result == target,
                "result: {:?}\ntarget: {:?}",
                result,
                target
            );
        }

        test_duration.exit();
    }
}
