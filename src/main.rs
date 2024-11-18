mod kernel;
mod app;
mod algorithm;
#[cfg(test)]
mod tests;
use algorithm::{hook_choose::user_hook::{self, user_hook::UserHook}, storage::storage::Storage, user_select::user_select::UserSelect};
use app::app::App;
use kernel::{run::Run};
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

    // Заполнение хранилища
    let mut storage = Storage::new();
    if let Err(e) = storage.load_from_json("C:\\Users\\klaim\\Desktop\\cdt-math\\src\\kernel\\storage\\storage.json") {
        eprintln!("Error loading JSON: {}", e);
        return;
    }

    let user_select = match UserSelect::load_from_json("C:\\Users\\klaim\\Desktop\\cdt-math\\src\\kernel\\storage\\user_select_storage.json") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading UserSelect data from JSON: {}", e);
            return;
        }
    };

    let mut user_hook: UserHook = UserHook::new();
    user_hook.eval(user_select.hook_type, user_select.lifting_mechanism_work_type, user_select.m_to_lift, &mut storage);

    user_hook.user_hook.paint();
}