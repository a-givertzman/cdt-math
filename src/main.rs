mod kernel;
mod app;
#[cfg(test)]
mod tests;
use app::app::App;
use kernel::run::Run;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
///
/// Application entry point
use kernel::crane_constructor::hook_chooser;
use kernel::crane_constructor::user::user_select::UserSelect;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    };

    //Запрос пользователя
    let user: UserSelect = UserSelect::new();

}
