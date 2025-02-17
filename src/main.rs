mod algorithm;
mod app;
mod infrostructure;
mod kernel;
#[cfg(test)]
mod tests;
use algorithm::{context::{context::Context, context_access::ContextRead}, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, entities::hook::Hook, hook_filter::hook_filter::HookFilter, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use infrostructure::client::choose_user_hook::ChooseUserHookQuery;
use kernel::{eval::Eval, link::Link, request::Request, run::Run, storage::storage::Storage, user_setup::user_hook::UserHook};
///
/// Application entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbgid = DbgId("main".into());
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    }
    let user_hook = UserHook::new(
    Request::<Hook>::new(|ctx: Context| -> Hook {
            let link: &Link = ctx.read();
            let query = ChooseUserHookQuery::new(vec![]);
            link.req(query)
        }),
    HookFilter::new(
        DynamicCoefficient::new(
                SelectBettaPhi::new(
                    LiftingSpeed::new(
                        Context::new(InitialCtx::new(
                            &mut Storage::new(
                                "./src/tests/unit/kernel/storage/cache/test_3"
                                )
                            ).unwrap()
                        )
                    )
                )
            )
        )
    )
    .eval();
    Ok(())
}
