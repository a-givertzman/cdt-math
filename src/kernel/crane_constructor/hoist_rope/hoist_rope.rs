
use crate::kernel::{crane_constructor::{hoisting_tackle::hoisting_tackle::HoistingTackle, user::{self, user_select::UserSelect}}, storage::storage::{Storage, Value}};
///
/// Класс, реализующий выбор каната
/// - S - максимальное усилие, возникающее в канате при подъёме номинального груза
/// - cable_count - количество канатов, сходящих с крюковой подвески 
/// - m_to_lift - масса на крюке
/// - multiplicity_of_polispast - кратностью полиспаста
/// - hook_weight - масса грузозахватного или грузозахватных органов, не включенные в массу на крюке
/// - rope_type - тип каната
/// - work_area - окружающая среда крана
/// - hook_work_type - режим работы грузозахватного органа
/// - rejecting_blocks - количество отклоняющих блоков для полиспаста
/// - F - необходимое минимальное разрывное усилие
/// - rope_diametr - диаметр каната, введенный пользователем
/// 
pub struct HoistRope{
    ropes_list: Vec<Vec<String>>,
    dbgid: String,
    cable_count: f64,
    m_to_lift: f64,
    hook_weight: f64,
    S: f64,
    multiplicity_of_polispast: f64,
    rope_type: String,
    work_area: String,
    hook_work_type: String,
    rejecting_blocks: f64,
    F: f64,
    rope_diametr: f64,
    twisting_method: String,
    balance_degree: String,
}
//
//
//
impl HoistRope{
    ///
    /// Метод создание экземпляра класса HoistRope
    /// - hoisting_tackle - параметры полиспаста
    /// - user_select - характеристика для расчёта, выбранными пользователем
    /// 
    pub fn new(hoisting_tackle: &HoistingTackle, user_select: &UserSelect) -> Self{
        Self{
            ropes_list: Vec::new(),
            dbgid: hoisting_tackle.dbgid.clone(),
            balance_degree: user_select.balance_degree.clone(),
            twisting_method: user_select.rope_twisting_method.clone(),
            rope_diametr: user_select.rope_diametr,
            F: 0.0,
            rejecting_blocks: user_select.rejecting_blocks.clone(),
            hook_work_type: user_select.m_work_type.clone(),
            work_area: user_select.crane_type_area.clone(),
            rope_type: user_select.rope_type.clone(),
            cable_count: hoisting_tackle.cable_count.clone(),
            hook_weight: hoisting_tackle.hook_summary_weight.clone(),
            m_to_lift: hoisting_tackle.m_to_lift.clone(),
            multiplicity_of_polispast: hoisting_tackle.multiplicity_of_polispast.clone(),
            S: 0.0,
        }
    }
    ///
    /// Метод расчета типа каната
    /// - rope_storage - экземляр класса-хранилища Storage, в котором хранится таблица канатов
    /// 
    pub fn eval(&mut self, rope_storage: &mut Storage){
        self.S = Self::s_get(self.m_to_lift,self.hook_weight,9.81,self.cable_count,Self::n_get(self.multiplicity_of_polispast, self.rejecting_blocks));
        self.F = Self::F_get(self.S, Self::z_select(&self));
        self.ropes_list = Self::filter(&self, rope_storage);
    }
    ///
    /// Метод расчёта максимального усилия, возникающего в канате при подъёме номинального груза
    /// - M - масса на крюке
    /// - m - масса грузозахватного органа
    /// - n - кратность полиспаста
    /// - N - коэффициент полезного действия полиспаста
    /// 
    pub fn s_get(M: f64, m: f64, g: f64, n: f64, N: f64) -> f64{
        (M+m)*g/(n*N)
    }
    ///
    /// Метод расчёта коэффициента полезного действия полиспаста
    /// multiplicity_of_polispast - кратность полиспаста
    /// rejecting_blocks - количество отклоняющих блоков для полиспаста
    /// 
    pub fn n_get(multiplicity_of_polispast: f64,rejecting_blocks: f64) -> f64{
        f64::powf(0.985,rejecting_blocks)*(1.0 - f64::powf(0.98, multiplicity_of_polispast))/((1.0-0.98)*multiplicity_of_polispast)
    }
    ///
    /// Метод выбора коэффициента запаса каната
    ///
    fn z_select(&self) -> f64{
        if self.work_area == "агрессивная" || self.work_area == "сильно агрессивная" {
            return 5.6;
        } else{
            match self.hook_work_type.as_str() {
                "M1" => {
                    return 3.15;
                }
                "M2" => {
                    return 3.35;
                }
                "M3" => {
                    return 3.55;
                }
                "M4" => {
                    return 4.0;
                }
                "M5" => {
                    return 4.5;
                }
                "M6" => {
                    return 5.6;
                }
                "M7" => {
                    return 7.1;
                }
                "M8" => {
                    return 9.0;
                }
                _ =>{

                }
            }
        }
        0.0
    }
    ///
    /// Метод расчёта необходимого минимального разрывного усилия
    /// - S - максимальное усилие, возникающее в канате при подъёме номинального груза
    /// - Z - коэффициент запаса каната
    /// 
    fn F_get(S:f64, Z: f64) -> f64{
        S*Z
    }
    ///
    /// Метод фильтрования канатов по разрывному усилиния каната
    /// - rope_storage - экземляр класса-хранилища Storage, в котором хранится таблица канатов
    /// 
    fn filter(&self, ropes_storage: &mut Storage) -> Vec<Vec<String>>{
        let mut res_ropes: Vec<Vec<String>> = Vec::new();
        println!("F: {}", self.F);
        println!("{}",self.rope_type);
        if let Some(value) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}", self.rope_type.trim())) {
            if let Value::NextMap(map) = value {
                for (STO,_) in map{
                    if let Some(val) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}",self.rope_type.trim(),STO)){
                        if let Value::NextMap(types) = val{
                            for (key_iso,_) in types{
                                let mut tmp_rope: Vec<String> = Vec::new();               

                                if let Some(va) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/диаметр каната/",self.rope_type.trim(),STO,key_iso)){
                                    if let Value::Data(diametr) = va{
                                        if *diametr==self.rope_diametr{
                                            if let Some(gr) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/разрывное усилие",self.rope_type.trim(),STO,key_iso)){
                                                if let Value::NextMap(groups) = gr{
                                                    for (group,_) in groups{
                                                        if let Some(t) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/разрывное усилие/{}/",self.rope_type.trim(),STO,key_iso,group)){
                                                            if let Value::Data(F) = t{
                                                                if *F>=self.F{
                                                                    if let Some(g) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/способ свивки каната/",self.rope_type.trim(),STO,key_iso)){
                                                                        if let Value::String(sposob) = g{
                                                                            if *sposob == self.twisting_method{
                                                                                if let Some(i) = ropes_storage.get(&format!("конструкции/канат/тип сердечника/{}/{}/{}/степень уравновешенности/",self.rope_type.trim(),STO,key_iso)){
                                                                                    if let Value::String(pow) = i{
                                                                                        if *pow == self.balance_degree{
                                                                                            tmp_rope.push(STO.to_string());
                                                                                            tmp_rope.push(self.rope_type.to_string());
                                                                                            tmp_rope.push(diametr.to_string());
                                                                                            tmp_rope.push(group.to_string());
                                                                                            
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
                                res_ropes.push(tmp_rope);
                            }
                        } 
                    }
                }
            }
        } else {
            
            println!("Path not found for hook selection.");
        }
        for rope in res_ropes.iter(){
            println!("{:?}",rope);
            log::debug!("{}.filter | Ropes: {:?}", self.dbgid,rope);
        }
        res_ropes
    }
    
}