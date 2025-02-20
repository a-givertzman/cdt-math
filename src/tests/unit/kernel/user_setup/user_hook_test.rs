#[cfg(test)]

mod user_hook {
    use std::{sync::{Arc, Once}, time::Duration};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        algorithm::{
            context::{context::Context, context_access::ContextRead, ctx_result::CtxResult},
            dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::hook::Hook,
            hook_filter::{hook_filter::HookFilter, hook_filter_ctx::HookFilterCtx}, initial::Initial,
            initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed,
            select_betta_phi::select_betta_phi::SelectBettaPhi,
        },
        infrostructure::client::{choose_user_hook::{ChooseUserHookQuery, ChooseUserHookReply}, query::Query},
        kernel::{eval::Eval, link::Link, mok_user_reply::mok_user_reply::MokUserReply, request::Request, storage::storage::Storage, user_setup::{user_hook::UserHook, user_hook_ctx::UserHookCtx}}
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
    /// Testing such functionality / behavior
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
                Hook {
                    gost: "GOST 34567-85".to_string(),
                    r#type: "Forged".to_string(),
                    load_capacity_m13: 25.0,
                    load_capacity_m46: 23.0,
                    load_capacity_m78: 21.0,
                    shank_diameter: 85.0,
                },
            )
        ];
        let (local, remote) = Link::split(dbg);
        let mut mok_user_reply = MokUserReply::new(dbg, remote);
        let mok_user_reply_handle = mok_user_reply.run().unwrap();
        let local = Arc::new(local);
        for (step, cache_path, target) in test_data {
            let result = UserHook::new(
                Request::<ChooseUserHookReply>::new(|ctx: &Context| {
                    let variants: &HookFilterCtx = ctx.read();
                    let query = Query::ChooseUserHook(ChooseUserHookQuery::test(variants.result.clone()));
                    ctx.link.req(query).expect("{}.req | Error to send request")
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
                                    local.clone(),
                                    )
                                )
                            )
                        )
                    )
                )
            ).eval();
            match result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<UserHookCtx>::read(&result)
                        .result
                        .clone();
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
