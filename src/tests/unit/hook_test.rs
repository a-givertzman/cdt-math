#[cfg(test)]

mod hook {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::hook_chooser::hook::Hook;
    use crate::kernel::crane_constructor::hook_chooser::param_comp::ParamComp;
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

        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/", 0.4);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/", 0.3);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/", 12.0);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/", 12.0);
        
        storage.set("конструкции/подшипники/название/8100Н/статическая грузоподъемность/", 11800.0);
        storage.set("конструкции/подшипники/название/8101Н/статическая грузоподъемность/", 12900.0);
        storage.set("конструкции/подшипники/название/8100Н/наружный диаметр/", 11.0);
        storage.set("конструкции/подшипники/название/8101Н/наружный диаметр/", 11.0);



        let test_data =[
            ((0.1,"крюк однорогий","M1"), vec!["1","2"]),
            ((0.2,"крюк однорогий","M1"), vec!["1","2"]),
            ((0.3,"крюк однорогий","M1"), vec!["1","2"]),
            ((0.6,"крюк однорогий","M1"), vec!["3","4"]),
            ((0.55,"крюк однорогий","M1"), vec!["3","4"]),
            ((0.62,"крюк однорогий","M1"), vec!["3","4"]),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = Hook::weight_check(&ParamComp{
                _m_to_lift: value.0,
                _hook_type: value.1.to_string(),
                _m_work_type: value.2.to_string(),
                _fmg: ParamComp::fmg(3.0, value.0, 9.8)
            }, &mut storage);
            let mut i:usize = 0;
            for value in result.iter()  {
                assert!(value[0] == target[i], "result: {:?}\ntarget: {:?}", value[0], target[i]);
                i+=1;
            }
            
        }
        
        test_duration.exit();
    }

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

        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/", 0.4);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/", 0.3);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/", 12.0);
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/", 12.0);
        
        storage.set("конструкции/подшипники/название/8100Н/статическая грузоподъемность/", 11800.0);
        storage.set("конструкции/подшипники/название/8101Н/статическая грузоподъемность/", 12900.0);
        storage.set("конструкции/подшипники/название/8100Н/наружный диаметр/", 11.0);
        storage.set("конструкции/подшипники/название/8101Н/наружный диаметр/", 11.0);



        let test_data =[
            ((0.1,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            ((0.2,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            ((0.3,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            ((0.15,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            ((0.25,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            ((0.33,"крюк однорогий","M1"), vec!["8100Н", "8100Н", "8101Н", "8101Н"]),
            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = Hook::bearing_check(&ParamComp{
                _m_to_lift: value.0,
                _hook_type: value.1.to_string(),
                _m_work_type: value.2.to_string(),
                _fmg: ParamComp::fmg(3.0, value.0, 9.8)
            }, &mut storage,&vec!["2".to_string(),"1".to_string()]);
            assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
        }
        
        test_duration.exit();
    }


}
