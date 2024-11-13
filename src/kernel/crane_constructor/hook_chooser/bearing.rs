use std::io;

use crate::kernel::crane_constructor::hook_chooser::all_bearings::AllBearings;

///
/// Класс, реализующий выбор подходящего подшипника из списка подходящих
/// - dbgid - debug id
/// - all_bearings - вектор подходящих подшипников
/// - bearing_name - имя подходящего подшипника
/// ор
pub struct Bearing{
    dbgid: String,
    all_bearings: Vec<String>,
    bearing_name: String,
}
//
//
//
impl Bearing{
    ///
    /// Метод создания экземпляра класса Bearing
    /// - all_bearings - вектор всех подходящих подшипников
    /// 
    pub fn new(all_bearings: &AllBearings) -> Self{        
        Self {  dbgid: String::from(format!("{}/Bearing",all_bearings.dbgid)),
                all_bearings: all_bearings.res_bearings.clone(),
                bearing_name: String::new() 
             }
    }
    ///
    /// Метод для выбора подшипника из списка предложенных
    /// - bearings - все подходящие подшпники
    /// 
    pub fn eval(&mut self,bearings: Vec<String>){
        self.bearing_name = Self::bearing_select(&self, bearings);
    }
    ///
    /// Метод выбора подшипников из предложенных
    /// - bearings - вектор в котором лежат все подходящие подшипники
    ///
    fn bearing_select(&self, bearings: Vec<String>) -> String {
        println!("{:?}", bearings);

        if bearings.len() != 0 {
            log::debug!("{}.bearing_select | Which bearing do you choose?",self.dbgid);
            let mut counter: usize = 0;

            // Печать вариантов выбора
            for value in bearings.iter() {
                log::debug!("{}.bearing_select | Bearing {}: {:?}", self.dbgid, counter,value);
                counter += 1;
            }

            let mut user_select = String::new();

            // Чтение выбора пользователя
            match io::stdin().read_line(&mut user_select) {
                Ok(_) => {}
                Err(e) => {
                    println!("Input error! {}", e);
                    return String::new(); // Возврат пустого вектора в случае ошибки
                }
            }

            bearings[1].clone()
        } else {
            log::debug!("{}.bearing_select | There is no right bearing for your hook",self.dbgid);
            String::new()
        }
    }
}