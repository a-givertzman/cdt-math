#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{context::{context::Context, context_access::ContextRead, ctx_result::CtxResult}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::hook::Hook, hook_filter::{hook_filter::HookFilter, hook_filter_ctx::HookFilterCtx}, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi}, infrostructure::client::choose_user_hook::ChooseUserHookQuery, kernel::{eval::Eval, link::Link, request::Request, storage::storage::Storage, user_setup::{user_hook::UserHook, user_hook_ctx::UserHookCtx}}};
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
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = "test";
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                "./src/tests/unit/kernel/storage/cache/test_1",
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
        for (step,cache_path,target) in test_data.iter() {
            let (local, _remote) = Link::split(dbgid);
            let result = UserHook::new(
                Request::<Hook>::new(|ctx: Context| -> Hook {
                    let link: &Link = ctx.read();
                    let variants: &HookFilterCtx = ctx.read();
                    let query = ChooseUserHookQuery::new(variants.result.clone());
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
                                    local.into()
                                        )
                                )
                            )
                        )
                    )
                )
            ).eval();
            match &result {
                CtxResult::Ok(result) => {
                    let result = ContextRead::<UserHookCtx>::read(result)
                        .result
                        .clone();
                    assert!(
                        result == *target,
                        "step {} \nresult: {:?}\ntarget: {:?}",
                        step,
                        result,
                        target
                    );
                }
                CtxResult::Err(_) => {},
                CtxResult::None => {},
                _ => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            }
        } 
        test_duration.exit();
    }
}
