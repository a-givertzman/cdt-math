use crate::{algorithm::{bearing_choose::bearing::Bearing, force_of_gravity::force_of_gravity::ForceOfGravity, hook_choose::user_hook::user_hook::UserHook, storage::storage::Storage}, kernel::{dbgid::dbgid::DbgId, entities::value::Value}};


pub struct BearingFilter{
    pub(crate) dbgid: DbgId,
    pub(crate) user_hook: UserHook,
    pub(crate) force_of_gravity: ForceOfGravity,
    pub(crate) filtered_bearings: Vec<Bearing>,
}
//
//
//
impl BearingFilter{
    ///
    /// Конструктор класса BearingFilter
    pub fn new(user_hook: &UserHook, force_of_gravity: ForceOfGravity) -> Self{
        Self { dbgid: DbgId(format!("BearingFilter")), filtered_bearings: Vec::new(), user_hook: user_hook.clone(), force_of_gravity }
    }
    ///
    /// Метод фильтрации подшипников 
    pub fn filter(&mut self, storage: &Storage) -> &Vec<Bearing> {
        let mut fmg: f64 = 0.0;
        match self.force_of_gravity.eval() {
            Ok(value) => fmg = value,
            Err(_) => todo!(),
        }
        if let Some(value) = storage.get("конструкции/подшипники/название")
        {
            if let Value::NextMap(bearings_map) = value {
                for (bearing_name, _) in bearings_map {
                    if let Some(Value::Data(static_load)) = storage.get(&format!("конструкции/подшипники/название/{}/статическая грузоподъемность/",bearing_name)) {
                        if fmg <= *static_load {
                            if let Some(Value::Data(d_out)) = storage.get(&format!("конструкции/подшипники/название/{}/наружный диаметр/",bearing_name)) {
                                if d_out <= &self.user_hook.user_hook.d_tail{
                                    self.filtered_bearings.push(Bearing::new(bearing_name.to_string(), *d_out));
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("Path not found for bearing selection.");
        }
        &self.filtered_bearings
    }

}