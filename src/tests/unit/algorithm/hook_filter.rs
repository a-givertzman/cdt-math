#[cfg(test)]

mod hook {
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{
        sync::Once,
        time::{Duration, Instant},
    };
    use testing::stuff::max_test_duration::TestDuration;

    use crate::algorithm::{hook_choose::{filter::hook_filter::HookFilter, hook::Hook}, storage::storage::Storage, user_select::user_select::UserSelect};
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
    fn test_weightcheck() {
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
            Ok(12.0),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/диаметр хвостовика/",
            Ok(15.0),
        );
        storage.set(
            "конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/диаметр хвостовика/",
            Ok(17.0),
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

        //Заполнение хранилища выбор пользователя
        let mut user_select_storage = Storage::new();
        user_select_storage.set("грузоподъемность/", Ok(0.1));
        user_select_storage.set("класс подъема/", Err("HC3".to_string()));
        user_select_storage.set("комбинация нагрузок/", Err("A1".to_string()));
        user_select_storage.set("тип привода/", Err("HD3".to_string()));
        user_select_storage.set("номинальная скорость подъема механизма/", Ok(1.0));
        user_select_storage.set("замедленная скорость подъема механизма/", Ok(1.0));
        user_select_storage.set("режим работы механизма/", Err("M1".to_string()));
        user_select_storage.set("тип крюка/", Err("крюк однорогий".to_string()));
        user_select_storage.set("тип грузозахватного органа механизма подъёма/",Err("съёмный электрогидравлический грейфер".to_string()),);
        user_select_storage.set( "грузоподъемность грузозахватного органа механизма подъёма/", Ok(0.3),);

        //Запрос пользователя
        let user_select: UserSelect = UserSelect::new(&user_select_storage);

        let test_data = [
            (
                HookFilter::new(&user_select), 4
            )
        ];

        for (mut value, target) in test_data.into_iter() {
            let result = value.filter(&mut storage).clone();
            let mut i: usize = 0;
            let mut i: usize = 0;
            assert!(result.len() == target);
        }
        test_duration.exit(); 
    }
}