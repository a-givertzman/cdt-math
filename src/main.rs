mod kernel;
mod app;
mod algorithm;
#[cfg(test)]
mod tests;
use algorithm::storage::storage::Storage;
use app::app::App;
use kernel::run::Run;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
///
/// Application entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    }
    let mut hooks_storage = Storage::new("src\\kernel\\storage\\construction.hooks.json");  
    match hooks_storage.load("type") {
        Ok(value) => log::debug!("{}.load | Found: {}",hooks_storage.dbgid,value),
        Err(err) => log::error!("{}.load | Some Error: {:#?}",hooks_storage.dbgid, err)
    }
    match hooks_storage.store("type.one-horned.sequence_number.1.capacity_M2", 0.2){
        Ok(_) => log::debug!("{}.store | Value succesful stored!",hooks_storage.dbgid),
        Err(err) => log::error!("{}.load | Some Error: {:#?}",hooks_storage.dbgid, err)
    }
    Ok(())
}

