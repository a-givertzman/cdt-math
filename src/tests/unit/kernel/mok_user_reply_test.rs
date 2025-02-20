#[cfg(test)]

mod mok_user_reply {
    use std::{sync::Once, time::Duration};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::entities::hook::Hook, infrostructure::client::{choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query}, kernel::{link::Link, mok_user_reply::mok_user_reply::MokUserReply}};
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
    /// Testing 'run' method
    #[test]
    fn run() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = "test";
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                ChooseUserHookQuery::test(vec![
                    Hook { 
                        gost: "".into(),
                        r#type: "".into(),
                        load_capacity_m13: 0.1,
                        load_capacity_m46: 0.2,
                        load_capacity_m78: 0.3,
                        shank_diameter: 0.4,
                    }
                ]),
                Hook {
                    gost: "GOST 34567-85".into(),
                    r#type: "Forged".into(),
                    load_capacity_m13: 25.0,
                    load_capacity_m46: 23.0,
                    load_capacity_m78: 21.0,
                    shank_diameter: 85.0,
                }
            )
        ];
        let (local, remote) = Link::split(dbg);
        let mut user = MokUserReply::new(dbg, remote);
        let _handle = user.run().unwrap();
        for (step, query, target) in test_data {
            let query = Query::ChooseUserHook(query);
            let result: ChooseUserHookReply = local.req(query).unwrap();
            assert!(result.choosen == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        user.exit();
        for (_, h) in _handle {
            h.join().unwrap()
        }
        test_duration.exit();
    }
    
}
