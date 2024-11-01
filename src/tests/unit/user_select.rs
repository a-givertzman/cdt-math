#[cfg(test)]

mod user_select {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::{crane_constructor::user::{self, user_select::{self, UserSelect}}, storage::storage::Storage};
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
    fn test_get_fmg() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();


        let mut user_select_storage = Storage::new();

        user_select_storage.set("грузоподъемность/",Ok(0.1));
        user_select_storage.set("класс подъема/",Err("HC3".to_string()));
        user_select_storage.set("комбинация нагрузок/",Err("A1".to_string()));
        user_select_storage.set("тип привода/",Err("HD3".to_string()));
        user_select_storage.set("номинальная скорость подъема механизма/",Ok(1.0));
        user_select_storage.set("замедленная скорость подъема механизма/",Ok(1.0));
        user_select_storage.set("режим работы механизма/",Err("M1".to_string()));
        user_select_storage.set("тип крюка/", Err("крюк однорогий".to_string()));
        user_select_storage.set("тип грузозахватного органа механизма подъёма/",Err("съёмный электрогидравлический грейфер".to_string()));
        user_select_storage.set("грузоподъемность грузозахватного органа механизма подъёма/",Ok(0.3));


        let test_data_1 =[
            (("класс подъема/"), "HC3"),
            (("комбинация нагрузок/"), "A1"),
            (("тип привода/"), "HD3"),
            (("режим работы механизма/"), "M1"),
            (("тип крюка/"), "крюк однорогий"),
            (("тип грузозахватного органа механизма подъёма/"), "съёмный электрогидравлический грейфер"),
        ];

        let test_data_2 =[
            (("номинальная скорость подъема механизма/"), 1.0),
            (("замедленная скорость подъема механизма/"), 1.0),
            (("грузоподъемность грузозахватного органа механизма подъёма/"), 0.3),
            (("грузоподъемность/"), 0.1),
        ];

        

        let user = UserSelect::new(user_select_storage);


        let result_1 =[
            user.lift_class,
            user.load_comb,
            user.drive_type,
            user.m_work_type,
            user.hook_type,
            user.cargo_name,
        ];

        let result_2 =[
            user.vhmax,
            user.vhcs,
            user.cargo_weight,
            user.m_to_lift,

        ];

        let mut counter = 0;
        for (value,target) in test_data_1.into_iter(){
            assert!(result_1[counter] == target, "result: {:?}\ntarget: {:?}", result_1[counter], target);
            counter+=1;
        }

        counter = 0;
        for (value,target) in test_data_2.into_iter(){
            assert!(result_2[counter] == target, "result: {:?}\ntarget: {:?}", result_2[counter], target);
            counter+=1;
        }

        test_duration.exit();

    }
    

}
