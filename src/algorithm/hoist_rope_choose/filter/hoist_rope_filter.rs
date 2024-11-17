use crate::{algorithm::{hoist_rope_choose::hoist_rope::HoistRope, minimum_breaking_force::{self, minimum_breaking_force::MinimumBreakingForce}, storage::storage::Storage}, kernel::{dbgid::dbgid::DbgId, entities::{hoist_rope_balance_degree::{self, HoistRopeBalanceDegree}, hoist_rope_twisting_method::{self, HoistRopeTwistingMethod}, hoist_rope_type::HoistRopeType, mechanism_work_type::MechanismWorkType, value::Value}}};

///
/// Класс фильтрации канатов под пользовательские характеристики
/// - 'filtered_hoist_ropes' - вектор отфильтрованных (экемпляр класса [HoistRope])
pub struct HoistRopeFilter{
    dbgid: DbgId,
    filtered_hoist_ropes: Vec<Vec<HoistRope>>,
    minimum_breaking_force: MinimumBreakingForce,
}
//
//
//
impl HoistRopeFilter{
    ///
    /// Конструктор класса HoistRopeFilter
    pub fn new() -> Self{
        Self { dbgid: DbgId(format!("HoistRopeFilter")), filtered_hoist_ropes: Vec::new(), minimum_breaking_force: MinimumBreakingForce::new() }
    }
    ///
    /// Метод фильтрации
    /// - rope_storage - экземляр класса-хранилища Storage, в котором хранится таблица канатов
    pub fn filter(&mut self, storage: &mut Storage,m_to_lift: f64, hook_weight: f64, rejecting_blocks: f64, mechanism_work_type: MechanismWorkType, hoist_rope_type: HoistRopeType,hoist_rope_diametr: f64,hoist_rope_twisting_method: HoistRopeTwistingMethod,hoist_rope_balance_degree: HoistRopeBalanceDegree) -> Vec<Vec<HoistRope>>{
        let F_min = self.minimum_breaking_force.eval(m_to_lift, hook_weight, rejecting_blocks, mechanism_work_type);
        let binding = hoist_rope_type.clone().to_string();
        let hoist_r_t = binding.trim();
        if let Some(value) = storage.get(&format!("конструкции/канат/тип сердечника/{}", hoist_r_t.trim())) {
            if let Value::NextMap(map) = value {
                for (STO,_) in map{
                    if let Some(val) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}",hoist_r_t.trim(),STO)){
                        if let Value::NextMap(types) = val{
                            for (key_iso,_) in types{
                                let mut tmp_rope: Vec<HoistRope> = Vec::new();
                                if let Some(va) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/диаметр каната/",hoist_r_t,STO,key_iso)){
                                    if let Value::Data(diametr) = va{
                                        if *diametr==hoist_rope_diametr{
                                            if let Some(gr) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/разрывное усилие",hoist_r_t,STO,key_iso)){
                                                if let Value::NextMap(groups) = gr{
                                                    for (group,_) in groups{
                                                        if let Some(t) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/разрывное усилие/{}/",hoist_r_t,STO,key_iso,group)){
                                                            if let Value::Data(F) = t{
                                                                if *F>=F_min{
                                                                    if let Some(g) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/способ свивки каната/",hoist_r_t,STO,key_iso)){
                                                                        if let Value::String(sposob) = g{
                                                                            if *sposob == hoist_rope_twisting_method.to_string().trim(){
                                                                                if let Some(i) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/степень уравновешенности/",hoist_r_t,STO,key_iso)){
                                                                                    if let Value::String(pow) = i{
                                                                                        if *pow == hoist_rope_balance_degree.to_string(){
                                                                                            if let Some(i) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/площадь сечения каната/",hoist_r_t,STO,key_iso)){
                                                                                                if let Value::Data(the_cross_sectional_area_of_the_rope) = i{
                                                                                                    if let Some(i) = storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/удельная масса каната/",hoist_r_t,STO,key_iso)){
                                                                                                        if let Value::Data(specific_gravity_of_the_rope) = i{
                                                                                                            tmp_rope.push(HoistRope::new(STO.to_string(), *diametr, hoist_rope_type.clone(), group.to_string(), *F, *the_cross_sectional_area_of_the_rope, *specific_gravity_of_the_rope));
        
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self.filtered_hoist_ropes.push(tmp_rope);
                            }
                        } 
                    }
                }
            }
        } else {
            
            println!("Path not found for hook selection.");
        }
        for filtered_hoist_rope in self.filtered_hoist_ropes.iter(){
            for hoist_rope in filtered_hoist_rope{
                log::debug!("{}.filter | Ropes: {:?}", self.dbgid,hoist_rope);
            }
        }
        self.filtered_hoist_ropes.clone()
    }
}