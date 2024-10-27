mod kernel;
mod app;
#[cfg(test)]
mod tests;
use app::app::App;
use kernel::crane_constructor::hook_chooser::cargo::Cargo;
use kernel::crane_constructor::hook_chooser::hook::Hook;
use kernel::run::Run;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
///
/// Application entry point
use kernel::crane_constructor::hook_chooser::middle_din_coeff::MiddDinCoeff;
use kernel::crane_constructor::user::user_select::UserSelect;
use kernel::crane_constructor::hook_chooser::din_coeff::DinCoeff;
use kernel::crane_constructor::hook_chooser::param_comp::ParamComp;
use kernel::storage::storage::Storage;
use kernel::crane_constructor::hoisting_tackle::hoisting_tackle::hoisting_tackle;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = "config.yaml";
    let mut app = App::new(path);
    if let Err(err) = app.run() {
        log::error!("main | Error: {:#?}", err);
    };

    //Запрос пользователя
    let user: UserSelect = UserSelect::new();

    // Промежуточные коэфф для дин расчета
    //let mid_din_c: MiddDinCoeff = MiddDinCoeff::new(UserSelect::new());

    // Расчет динамического коэффициента
    //let din_coeff: DinCoeff = DinCoeff::new(MiddDinCoeff::new(UserSelect::new()));

    // Готовые значения для выбора крюка
    //let comp_param: ParamComp = ParamComp::new(DinCoeff::new(MiddDinCoeff::new(UserSelect::new())));


    // Заполнение хранилища пока только крюки
    let mut storage = Storage::new();

    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/", 0.4);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/", 0.5);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/грузоподъемность/M1/", 0.63);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/грузоподъемность/M1/", 0.8);

    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/", 12.0);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/", 12.0);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/диаметр хвостовика/", 15.0);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/диаметр хвостовика/", 17.0);
    
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/масса заготовки/", 0.2);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/масса заготовки/", 0.25);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/масса заготовки/", 0.4);
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/масса заготовки/", 0.6);

    storage.set("конструкции/подшипники/название/8100Н/статическая грузоподъемность/", 11800.0);
    storage.set("конструкции/подшипники/название/8101Н/статическая грузоподъемность/", 12900.0);
    storage.set("конструкции/подшипники/название/8100Н/наружный диаметр/", 24.0);
    storage.set("конструкции/подшипники/название/8101Н/наружный диаметр/", 26.0);

    storage.set("конструкции/подшипники/название/8102Н/статическая грузоподъемность/", 14000.0);
    storage.set("конструкции/подшипники/название/8103Н/статическая грузоподъемность/", 16600.0);
    storage.set("конструкции/подшипники/название/8102Н/наружный диаметр/", 28.0);
    storage.set("конструкции/подшипники/название/8103Н/наружный диаметр/", 30.0);

    // Выбор крюка
    let hook = Hook::new(ParamComp::new(DinCoeff::new(MiddDinCoeff::new(&user))), &mut storage);
    
    //Выбор грузозахватного органа относительно выбора пользователя крюк/другое откуда идут дальнейшие расчёты
    if !user.cargo_name.eq(""){
        let cargo = Cargo::new(&user,&hook);
        // Выбор полиспаста
        let _hoisting_tackle: hoisting_tackle = hoisting_tackle::new(&user, &hook); 
    }

    // Выбор полиспаста
    let _hoisting_tackle: hoisting_tackle = hoisting_tackle::new(&user, &hook);


}
