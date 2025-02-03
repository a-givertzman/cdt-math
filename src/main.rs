mod algorithm;
mod app;
mod kernel;
#[cfg(test)]
mod tests;
use std::sync::{Arc, RwLock};

use algorithm::{context::context::Context, dynamic_coefficient::dynamic_coefficient::DynamicCoefficient, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed::LiftingSpeed, select_betta_phi::select_betta_phi::SelectBettaPhi};
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use kernel::{eval::Eval, run::Run, storage::storage::Storage};
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
    let dynamic_coefficient = DynamicCoefficient::new(
        SelectBettaPhi::new(
            LiftingSpeed::new(
                Arc::new(
                    RwLock::new(
                        Context::new(
                            InitialCtx::new(
                                &mut Storage::new(
                                                    "./src/tests/unit/kernel/storage/cache/test_8",))
            .unwrap())))).eval().unwrap()).eval().unwrap()).eval();
    Ok(())
}
