#[cfg(test)]

mod hook {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::kernel::crane_constructor::hook_chooser::{all_hooks::AllHooks, hook::Hook};
    use crate::kernel::crane_constructor::hook_chooser::param_comp::Param_to_compare;
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

        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/", Ok(0.4));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/", Ok(0.5));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/грузоподъемность/M1/", Ok(0.63));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/грузоподъемность/M1/", Ok(0.8));
    
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/", Ok(53.0));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/", Ok(12.0));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/диаметр хвостовика/", Ok(15.0));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/диаметр хвостовика/", Ok(17.0));
        
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/масса заготовки/", Ok(0.2));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/масса заготовки/", Ok(0.25));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/масса заготовки/", Ok(0.4));
        storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/масса заготовки/", Ok(0.6));




        let test_data =[
            ((0.1,"крюк однорогий","M1"), vec!["1","2","3","4"]),
            ((0.41,"крюк однорогий","M1"), vec!["2","3","4"]),
            ((0.51,"крюк однорогий","M1"), vec!["3","4"]),
            ((0.64,"крюк однорогий","M1"), vec!["4"]),            
        ];
           
        for (value,target) in test_data.into_iter(){

            let result = AllHooks::weight_check(&value.2.to_string(),value.0, &value.1.to_string(),
                &mut storage);
            let mut i:usize = 0;
            assert!(result.len()==target.len());
            for value in result.iter()  {
                assert!(value[0] == target[i], "result: {:?}\ntarget: {:?}", value[0], target[i]);
                i+=1;
            }
            
        }
        
        test_duration.exit();
    }

}