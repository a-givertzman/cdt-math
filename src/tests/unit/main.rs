mod kernel;
mod app;
#[cfg(test)]
mod tests;


use app::app::App;
use kernel::crane_constructor::hook_chooser::cargo::Cargo;
use kernel::crane_constructor::hook_chooser::hook::Hook;
use kernel::crane_constructor::hook_chooser::param_comp::Param_to_compare;
use kernel::crane_constructor::user::user_select::UserSelect;
use kernel::storage::storage::Storage;
//use kernel::crane_constructor::hoisting_tackle::hoisting_tackle::hoisting_tackle;
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

    // Промежуточные коэфф для дин расчета
    //let mid_din_c: MiddDinCoeff = MiddDinCoeff::new(UserSelect::new());

    // Расчет динамического коэффициента
    //let din_coeff: DinCoeff = DinCoeff::new(MiddDinCoeff::new(UserSelect::new()));

    // Готовые значения для выбора крюка
    //let comp_param: ParamComp = ParamComp::new(DinCoeff::new(MiddDinCoeff::new(UserSelect::new())));


    // Заполнение хранилища пока только крюки
    let mut storage = Storage::new();


    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/грузоподъемность/M1/", Ok(0.4));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/грузоподъемность/M1/", Ok(0.5));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/грузоподъемность/M1/", Ok(0.63));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/грузоподъемность/M1/", Ok(0.8));

    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/диаметр хвостовика/", Ok(12.0));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/диаметр хвостовика/", Ok(12.0));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/диаметр хвостовика/", Ok(15.0));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/диаметр хвостовика/", Ok(17.0));
    
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/1/масса заготовки/", Ok(0.2));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/2/масса заготовки/", Ok(0.25));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/3/масса заготовки/", Ok(0.4));
    storage.set("конструкции/крюки/тип крюка/крюк однорогий/ИСО/4/масса заготовки/", Ok(0.6));

    storage.set("конструкции/подшипники/название/8100Н/статическая грузоподъемность/", Ok(11800.0));
    storage.set("конструкции/подшипники/название/8101Н/статическая грузоподъемность/", Ok(12900.0));
    storage.set("конструкции/подшипники/название/8100Н/наружный диаметр/", Ok(24.0));
    storage.set("конструкции/подшипники/название/8101Н/наружный диаметр/", Ok(26.0));

    storage.set("конструкции/подшипники/название/8102Н/статическая грузоподъемность/", Ok(14000.0));
    storage.set("конструкции/подшипники/название/8103Н/статическая грузоподъемность/", Ok(16600.0));
    storage.set("конструкции/подшипники/название/8102Н/наружный диаметр/", Ok(28.0));
    storage.set("конструкции/подшипники/название/8103Н/наружный диаметр/", Ok(30.0));

    storage.set("конструкции/подшипники/название/8104Н/статическая грузоподъемность/", Ok(22400.0));
    storage.set("конструкции/подшипники/название/8105Н/статическая грузоподъемность/", Ok(30000.0));
    storage.set("конструкции/подшипники/название/8104Н/наружный диаметр/", Ok(35.0));
    storage.set("конструкции/подшипники/название/8105Н/наружный диаметр/", Ok(42.0));

    storage.set("конструкции/подшипники/название/8106Н/статическая грузоподъемность/", Ok(33500.0));
    storage.set("конструкции/подшипники/название/8107Н/статическая грузоподъемность/", Ok(39000.0));
    storage.set("конструкции/подшипники/название/8106Н/наружный диаметр/", Ok(47.0));
    storage.set("конструкции/подшипники/название/8107Н/наружный диаметр/", Ok(52.0));

    //Заполнение хранилища выбор пользователя
    let mut user_select_storage = Storage::new();

    user_select_storage.set("грузоподъемность/",Ok(0.1));
    user_select_storage.set("класс подъема/",Err("HC3".to_string()));
    user_select_storage.set("комбинация нагрузок/",Err("A1".to_string()));
    user_select_storage.set("тип привода/",Err("HD3".to_string()));
    user_select_storage.set("номинальная скорость подъема механизма/",Ok(1.0));
    user_select_storage.set("замедленная скорость подъема механизма/",Ok(1.0));
    user_select_storage.set("режим работы механизма/",Err("M1".to_string()));
    user_select_storage.set("тип крюка/", Err("крюк однорогий".to_string()));
    user_select_storage.set("тип грузозахватного органа механизма подъёма/",Err("съёмный электрогидравлический грейфер".to_string()));
    user_select_storage.set("грузоподъемность грузозахватного органа механизма подъёма/",Ok(0.3));

    //Запрос пользователя
    let user: UserSelect = UserSelect::new(user_select_storage);



    // Выбор крюка
    let hook = Hook::new(Param_to_compare::new(user), &mut storage);
    println!("{:?}",hook.hook);  
    println!("{:?}",hook.bearing);  


    //Выбор грузозахватного органа относительно выбора пользователя крюк/другое откуда идут дальнейшие расчёты
    // if !user.cargo_name.eq(""){
    //     let cargo = Cargo::new(&user,&hook);
    //     // Выбор полиспаста
    //     let _hoisting_tackle: hoisting_tackle = hoisting_tackle::new(&user, &hook); 
    // }

    // // Выбор полиспаста
    // let _hoisting_tackle: hoisting_tackle = hoisting_tackle::new(&user, &hook);


}