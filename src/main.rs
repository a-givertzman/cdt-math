mod kernel;
mod app;
#[cfg(test)]
mod tests;
use app::app::App;
use kernel::run::Run;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
///
/// Application entry point
use kernel::crane_constructor::hook_chooser::middle_din_coeff::MiddDinCoeff;
use kernel::crane_constructor::user::user_select::UserSelect;
use kernel::crane_constructor::hook_chooser::din_coeff::DinCoeff;
use kernel::crane_constructor::hook_chooser::param_comp::ParamComp;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    };

    //Запрос пользователя
    //let user: UserSelect = UserSelect::new();

    // Промежуточные коэфф для дин расчета
    //let mid_din_c: MiddDinCoeff = MiddDinCoeff::new(UserSelect::new());

    // Расчет динамического коэффициента
    //let din_coeff: DinCoeff = DinCoeff::new(MiddDinCoeff::new(UserSelect::new()));

    // Готовые значения для выбора крюка
    let comp_param: ParamComp = ParamComp::new(DinCoeff::new(MiddDinCoeff::new(UserSelect::new())));

}
