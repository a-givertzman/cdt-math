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
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    };
    let mut storage = Storage::new();
    if let Err(e) = storage.load_from_json("C:\\Users\\klaim\\Desktop\\cdt-math\\src\\kernel\\storage\\storage.json") {
        eprintln!("Error loading JSON: {}", e);
        return;
    }
    match storage.get_hook(1){
        Some(hook) => hook.paint(),
        None => todo!(),
    }
}
