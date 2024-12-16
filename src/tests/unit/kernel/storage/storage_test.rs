#[cfg(test)]
mod storage_test {
    use std::{collections::HashMap, sync::Once, time::Duration};
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
    /// Testing storing method
    #[test]
    fn test_store() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src\tests\unit\kernel\storage\cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data_1 = [
            (1,
            "type.one-horned.sequence_number.1.capacity_M1",
            0.2),
            (1,
            "type.one-horned.sequence_number.1.capacity_M2",
            0.4),
            (1,
            "type.one-horned.sequence_number.1.capacity_M3",
            1.0)
        ];
        for (step,path,target) in test_data_1{
            match hooks_storage.store(path, target){
                Ok(_) => assert!(true, "step {}: Value succesfully stored", step),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_2 = [
            (1,
            "type.one-horned.sequence_number.1.mechanism_work_type",
            "M1"),
            (1,
            "type.one-horned.sequence_number.1.driver_work_type",
            "HD1")
        ];
        for (step,path,target) in test_data_2{
            match hooks_storage.store(path, target){
                Ok(_) => assert!(true, "step {}: Value succesfully stored", step),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_3 = [
            (
                1,
                "constructions.hooks.1",
                {
                    let mut map = HashMap::new();
                    map.insert("One-horned".to_string(), 12.0);
                    map
                },
            )
        ];
        for (step,path,target) in test_data_3{
            match hooks_storage.store(path, target){
                Ok(_) => assert!(true, "step {}: Value succesfully stored", step),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_4 = [
            (
                1,
                "constructions.hooks.2",
                {
                    let mut map = HashMap::new();
                    map.insert("One-horned".to_string(), "M1");
                    map
                },
            )
        ];
        for (step,path,target) in test_data_4{
            match hooks_storage.store(path, target){
                Ok(_) => assert!(true, "step {}: Value succesfully stored", step),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_5 = [
            (
                1,
                "constructions.hooks.3",
                {
                    let mut vec = Vec::new();
                    vec.insert(0, "One-horned");
                    vec
                },
            )
        ];
        for (step,path,target) in test_data_5{
            match hooks_storage.store(path, target){
                Ok(_) => assert!(true, "step {}: Value succesfully stored", step),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        test_duration.exit();
    }
    ///
    /// 
    #[test]
    fn test_load() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src\tests\unit\kernel\storage\cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data_1 = [
            (1,
            "type.one-horned.sequence_number.1.capacity_M1",
            0.2),
            (1,
            "type.one-horned.sequence_number.1.capacity_M2",
            0.4),
            (1,
            "type.one-horned.sequence_number.1.capacity_M3",
            1.0)
        ];
        for (step,path,target) in test_data_1{
            match hooks_storage.load(path) {
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_2 = [
            (1,
            "type.one-horned.sequence_number.1.mechanism_work_type",
            "M1"),
            (1,
            "type.one-horned.sequence_number.1.driver_work_type",
            "HD1")
        ];
        for (step,path,target) in test_data_2{
            match hooks_storage.load(path) {
                Ok(result) => assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
            }
        }
        let test_data_3 = [
            (
                1,
                "constructions.hooks.1",
                {
                    let mut map = HashMap::new();
                    map.insert("One-horned".to_string(), 12.0);
                    map
                },
            )
        ];
        for (step, path, target) in test_data_3 {
            match hooks_storage.load(path) {
                Ok(result) => {
                    let deserialized_result: Result<HashMap<String, f64>, _> = serde_json::from_value(result.clone());
                    match deserialized_result {
                        Ok(parsed_result) => {
                            assert!(
                                parsed_result == target,
                                "step {} \nresult: {:?}\ntarget: {:?}",
                                step,
                                parsed_result,
                                target
                            );
                        }
                        Err(err) => {
                            log::error!(
                                "{}.load | Deserialization failed: {:#?} \nResult: {:?}",
                                dbgid,
                                err,
                                result
                            );
                        }
                    }
                }
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err),
            }
        }
        let test_data_4 = [
            (
                1,
                "constructions.hooks.2",
                {
                    let mut map = HashMap::new();
                    map.insert("One-horned".to_string(), "M1".to_string());
                    map
                },
            ),
        ];
        
        for (step, path, target) in test_data_4 {
            match hooks_storage.load(path) {
                Ok(result) => {
                    let deserialized_result: Result<HashMap<String, String>, _> = serde_json::from_value(result.clone());
                    match deserialized_result {
                        Ok(parsed_result) => {
                            assert!(
                                parsed_result == target,
                                "step {} \nresult: {:?}\ntarget: {:?}",
                                step,
                                parsed_result,
                                target
                            );
                        }
                        Err(err) => {
                            log::error!(
                                "{}.load | Deserialization failed: {:#?} \nResult: {:?}",
                                dbgid,
                                err,
                                result
                            );
                        }
                    }
                }
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err),
            }
        }
        let test_data_5 = [
            (
                1,
                "constructions.hooks.3",
                {
                    let mut vec = Vec::new();
                    vec.push("One-horned".to_string()); // Используем вектор
                    vec
                },
            ),
        ];
        for (step, path, target) in test_data_5 {
            match hooks_storage.load(path) {
                Ok(result) => {
                    // Попытка десериализации результата в Vec
                    let deserialized_result: Result<Vec<String>, _> = serde_json::from_value(result.clone());
                    match deserialized_result {
                        Ok(parsed_result) => {
                            assert!(
                                parsed_result == target,
                                "step {} \nresult: {:?}\ntarget: {:?}",
                                step,
                                parsed_result,
                                target
                            );
                        }
                        Err(err) => {
                            log::error!(
                                "{}.load | Deserialization failed: {:#?} \nResult: {:?}",
                                dbgid,
                                err,
                                result
                            );
                        }
                    }
                }
                Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err),
            }
        }
    }
}
