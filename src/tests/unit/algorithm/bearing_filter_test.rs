#[cfg(test)]

mod bearing_filter {
    use std::{sync::Once, time::Duration};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        algorithm::{
            context::{context::Context, context_access::ContextRead, ctx_result::CtxResult},
            initial::Initial, initial_ctx::initial_ctx::InitialCtx,
            bearing_filter::{bearing_filter::BearingFilter, bearing_filter_ctx::BearingFilterCtx},
            dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::bearing::Bearing,
            hook_filter::{hook_filter::HookFilter, hook_filter_ctx::HookFilterCtx},
            lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi,
        },
        infrostructure::client::{choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query},
        kernel::{eval::Eval, link::Link, mok_user_reply::mok_user_reply::MokUserReply, request::Request, storage::storage::Storage, user_setup::user_hook::UserHook}
    };
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
    /// Testing 'eval' method
    // #[test]
    #[tokio::test]
    async fn eval() {
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
                r#"./src/tests/unit/kernel/storage/cache/test_2"#,
                vec![
                    Bearing { 
                        name: "Bearing A".to_string(),
                        outer_diameter: 120.5,
                        inner_diameter: 60.2,
                        static_load_capacity: 1500.0,
                        height: 35.0 
                    }
                ],
            ),
            (
                2,
                r#"./src/tests/unit/kernel/storage/cache/test_3"#,
                vec![
                    Bearing { 
                        name: "Bearing A".to_string(),
                        outer_diameter: 120.5,
                        inner_diameter: 60.2,
                        static_load_capacity: 1500.0,
                        height: 35.0 
                    },
                    Bearing { 
                        name: "Bearing B".to_string(),
                        outer_diameter: 100.0,
                        inner_diameter: 50.0,
                        static_load_capacity: 1200.0,
                        height: 30.0 
                    },
                    Bearing { 
                        name: "Bearing C".to_string(),
                        outer_diameter: 140.0,
                        inner_diameter: 70.0,
                        static_load_capacity: 1800.0,
                        height: 40.0 
                    }
                ],
            )
        ];
        let (local, remote) = Link::split(dbg);
        let mut mok_user_reply = MokUserReply::new(dbg, remote);
        let mok_user_reply_handle = mok_user_reply.run().unwrap();
        for (step, cache_path, target) in test_data {
            let result = BearingFilter::new(
                UserHook::new(
                    local,
                    Request::<ChooseUserHookReply>::new(|ctx: &Context, link: &mut Link| {
                        let variants: &HookFilterCtx = ctx.read();
                        let query = Query::ChooseUserHook(ChooseUserHookQuery::test(variants.result.clone()));
                        link.req(query).expect("{}.req | Error to send request")
                    }),
                    HookFilter::new(
                        DynamicCoefficient::new(
                            SelectBettaPhi::new(
                                LiftingSpeed::new(
                                    Initial::new(
                                        Context::new(
                                                InitialCtx::new(
                                                    &mut Storage::new(
                                                        cache_path
                                                    )
                                                ).unwrap(),
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            ).eval();
            match result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<BearingFilterCtx>::read(&result)
                        .result
                        .clone();
                    println!("{:?}",result);
                    assert!(
                        result == target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    );
                }
                CtxResult::Err(err) => panic!("step {} \nerror: {:#?}", step, err),
                CtxResult::None => panic!("step {} \nerror: `UserHook` returns None", step),
            }
        }
        mok_user_reply.exit();
        for (_, h) in mok_user_reply_handle {
            h.join().unwrap();
        }
        test_duration.exit();
    }
}
