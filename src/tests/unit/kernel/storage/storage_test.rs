#[cfg(test)]
mod storage {
    use std::{collections::HashMap, fs::OpenOptions, io::BufReader, path::{Path, PathBuf}, sync::Once, time::Duration};
    use api_tools::debug::dbg_id::DbgId;
    use serde::Serialize;
    use serde_json::json;
    use testing::{entities::test_value::Value, stuff::max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::kernel::{storage::storage::Storage, str_err::str_err::StrErr};
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
    /// Returns content of JSON file
    fn read(dbgid: &DbgId, path: PathBuf) -> Result<serde_json::Value, StrErr> {
        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(|err| StrErr(format!("{}.read | Failed to open file: {:?}, error: {}", dbgid, path, err)))?;
        match serde_json::from_reader::<_, serde_json::Value>(BufReader::new(file)) {
            Ok(json_value) => {
                Ok(json_value)
            }
            Err(err) => Err(StrErr(format!("{}.read | Parse error: {} in the file: {:?}", dbgid, err, path))),
        }
    }
    ///
    /// Testing storing method on simple types
    #[test]
    fn store() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"./src/tests/unit/kernel/storage/cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (1,
            "type.one-horned.sequence_number.1.capacity_M1",
            Value::Double(0.2)),
            (1,
            "type.one-horned.sequence_number.1.capacity_M2",
            Value::Double(0.4)),
            (1,
            "type.one-horned.sequence_number.1.capacity_M3",
            Value::Double(1.0)),
            (1,
            "type.one-horned.sequence_number.1.mechanism_work_type",
            Value::String("M1".into())),
            (1,
            "type.one-horned.sequence_number.1.driver_work_type",
            Value::String("HD1".into())),
        ];
        for (step, path,target) in test_data {
            fn store_value(dbgid: &DbgId, step: usize, hooks_storage: &mut Storage, path: &str, value: impl Serialize) {
                match hooks_storage.store(path, value){
                    Ok(_) => log::debug!("{} | step {}: Value succesfully stored", dbgid, step),
                    Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
                }
            }
            match target {
                Value::Bool(value) => store_value(&dbgid, step, &mut hooks_storage, path, value),
                Value::Int(value) => store_value(&dbgid, step, &mut hooks_storage, path, value),
                Value::Real(value) => store_value(&dbgid, step, &mut hooks_storage, path, value),
                Value::Double(value) => store_value(&dbgid, step, &mut hooks_storage, path, value),
                Value::String(value) => store_value(&dbgid, step, &mut hooks_storage, path, value),
            }
        }
        test_duration.exit();
    }
    ///
    /// Testing storing method on Map<String, f64>
    #[test]
    fn store_map_str_f64() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("store_map_str_f64".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src\tests\unit\kernel\storage\cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (
                1,
                "constructions.map.f64",
                HashMap::from([
                    ("12.0", 12.0),
                    ("-14.1", -14.1),
                ]),
            )
        ];
        for (step,path,target) in test_data {
            match hooks_storage.store(path, target){
                Ok(_) => log::debug!("{} | step {}: Value succesfully stored", dbgid, step),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
        }
        }

        test_duration.exit();
    }
    ///
    /// Testing storing method on Map<String, String>
    #[test]
    fn store_map_str_str() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("store_map_str_str".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src\tests\unit\kernel\storage\cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (
                1,
                "constructions.hooks.1",
                HashMap::from([
                    ("12.0", "Value 12.0"),
                    ("-14.1", "Value -14.1"),
                ]),
            )
        ];
        for (step,path,target) in test_data {
            match hooks_storage.store(path, target){
                Ok(_) => log::debug!("{} | step {}: Value succesfully stored", dbgid, step),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
        }
        }
        test_duration.exit();
    }
    ///
    /// Testing store() on Vec<String>
    #[test]
    fn store_vec_str() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("store_map_int_str".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src/tests/unit/kernel/storage/cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (
                1,
                "constructions.vec.str",
                vec![
                    "Value 0",
                    "Value 1",
                    "Value 2",
                ],
            )
        ];
        for (step,path,target) in test_data {
            match hooks_storage.store(path, target){
                Ok(_) => log::debug!("{} | step {}: Value succesfully stored", dbgid, step),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
        }
        }
        test_duration.exit();
    }
    ///
    /// Testing store() on Vec<f64>
    #[test]
    fn store_vec_f64() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("store_map_int_str".into());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = r#"src/tests/unit/kernel/storage/cache"#;
        let mut hooks_storage = Storage::new(path);
        let test_data = [
            (
                1,
                "constructions.vec.f64",
                vec![
                    -0.223,
                    -0.10,
                    0.0,
                    0.10,
                    0.22,
                ],
            )
        ];
        for (step,key,target) in test_data {
            match hooks_storage.store(key, target.clone()){
                Ok(_) => log::debug!("{} | step {}: Value succesfully stored", dbgid, step),
                Err(err) => panic!("{} | step {},  Error: {:#?}", dbgid, step, err),
            }
            let result = read(&dbgid, PathBuf::from(path).join(key)).unwrap();
            assert!(result == json!(target), "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// 
    #[test]
    fn load() {
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
