mod algorithm;
mod app;
mod infrostructure;
mod kernel;
#[cfg(test)]
mod tests;
use algorithm::{context::context::Context, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, hook_filter::hook_filter::HookFilter, initial::Initial, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use kernel::{eval::Eval, link::Link, mok_user_reply::mok_user_reply::MokUserReply, run::Run, storage::storage::Storage};
use sal_sync::services::service::service::Service;
///
/// Application entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = DbgId("main".into());
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    }
    let cache_path = r#"src/assets/cache"#;
    let (local, remote) = Link::split(&dbg);
    let mut mok_user_reply = MokUserReply::new(&dbg, remote);
    let mok_user_reply_handle = mok_user_reply.run().unwrap();
    let _ = local;
    // let mut hooks_storage = Storage::new(cache_path);
    // match hooks_storage.load("type.one-horned.sequence_number.1.capacity_M2") {
    //     Ok(value) => log::debug!("{}.load | Found: {}", dbg, value),
    //     Err(err) => log::error!("{}.load | Some Error: {:#?}", dbg, err),
    // }
    // match hooks_storage.store("type.one-horned.sequence_number.1.capacity_M2", 0.2) {
    //     Ok(_) => log::debug!("{}.store | Value succesful stored!", dbg),
    //     Err(err) => log::error!("{}.store | Some Error: {:#?}", dbg, err),
    // }
    // match hooks_storage.load("type.one-horned.sequence_number.1.capacity_M2") {
    //     Ok(value) => log::debug!("{}.load | Found: {}", dbg, value),
    //     Err(err) => log::error!("{}.load | Some Error: {:#?}", dbg, err),
    // }
    match InitialCtx::new(&mut Storage::new(cache_path)) {
        Ok(initial) => {
            HookFilter::new(
                DynamicCoefficient::new(
                    SelectBettaPhi::new(
                        LiftingSpeed::new(
                            Initial::new(
                                Context::new(initial),
                            ),
                        ),
                    ),
                )
            ).eval();
        }
        Err(err) => {
            log::error!("{} | Error: {:?}", dbg, err);
        }
    }
    for (_id, h) in mok_user_reply_handle.into_iter() {
        h.join().unwrap();
    }
    Ok(())
}
