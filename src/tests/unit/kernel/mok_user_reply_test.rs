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
                ChooseUserHookReply::new(Hook {
                    gost: "ГОСТ Test".into(),
                    r#type: "Hook-type-Test".into(),
                    load_capacity_m13: 0.1,
                    load_capacity_m46: 0.2,
                    load_capacity_m78: 0.3,
                    shank_diameter: 0.4,
                })
            )
        ];
        let (local, remote) = Link::split("TestUser");
        let mut user = MokUserReply::new(dbg, remote);
        let _handle = user.run().unwrap();
        for (step, query, target) in test_data {
            let result: ChooseUserHookReply = local.req(Query::ChooseUserHook(query)).unwrap();
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    
}
