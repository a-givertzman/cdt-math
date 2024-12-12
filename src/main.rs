mod kernel;
mod app;
mod algorithm;
#[cfg(test)]
mod tests;
//
use api_tools::debug::dbg_id::DbgId;
use app::app::App;
use kernel::{run::Run, storage::storage::Storage};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
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
    let cache_path = "./assets/cache/";
    let mut hooks_storage = Storage::new(cache_path);
    match hooks_storage.load("type.one-horned.sequence_number.1.capacity_M1") {
        Ok(value) => log::debug!("{}.load | Found: {}", dbgid, value),
        Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
    }
    match hooks_storage.store("type.one-horned.sequence_number.1.capacity_M2", 0.2){
        Ok(_) => log::debug!("{}.store | Value succesful stored!", dbgid),
        Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
    }
    match hooks_storage.load("type.one-horned.sequence_number.1.capacity_M2") {
        Ok(value) => log::debug!("{}.load | Found: {}", dbgid, value),
        Err(err) => log::error!("{}.load | Some Error: {:#?}", dbgid, err)
    }
    Ok(())
}

