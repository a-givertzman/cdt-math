use crate::{algorithm::storage::storage::Storage, kernel::{dbgid::dbgid::DbgId, entities::{hook::Hook, mechanism_work_type::MechanismWorkType}}};
///
/// Класс, реалилующий фильтрацию крюков по грузоподъемности
/// - 'filtered_hooks' - вектор отфильтрованных крюков (экземпляр класса [Hook])
/// [reference to filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct HookFilter{
    dbgid: DbgId,
    filtered_hooks: Vec<Hook>,
}
//
impl HookFilter{
    ///
    /// Конструктор класса HookFilter
    pub fn new() ->Self{
        Self {  dbgid: DbgId(format!("Hook/HookFilter")),
                filtered_hooks: Vec::new(),
             }
    }
    ///
    /// Метод фильтрации БД крюков по грузоподъемности
    /// - 'hooks_storage' - хранилище крюков экземпляр класса [Storage]
    /// - 'hook_type' - тип крюка
    /// - 'mechanism_work_type' - тип работы механизмы подъема
    /// - 'm_to_lift' - масса на крюке
    /// [reference to filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    pub fn filter(&mut self,storage: &mut Storage,hook_type: String, mechanism_work_type: MechanismWorkType, m_to_lift: f64) -> Vec<Hook> {
        let mut hook_index = 0;
        loop {
            match storage.get_hook(hook_index){
                Some(hook) => {
                    let capacity_to_check = 
                    match mechanism_work_type{
                        MechanismWorkType::M1 => hook.capacity_M1,
                        MechanismWorkType::M2 => hook.capacity_M1,
                        MechanismWorkType::M3 => hook.capacity_M1,
                        MechanismWorkType::M4 => hook.capacity_M1,
                        MechanismWorkType::M5 => hook.capacity_M1,
                        MechanismWorkType::M6 => hook.capacity_M1,
                        MechanismWorkType::M7 => hook.capacity_M1,
                        MechanismWorkType::M8 => hook.capacity_M1,
                    };
                    if capacity_to_check >= m_to_lift{
                        self.filtered_hooks.push(hook.clone());
                    }
                },
                None => break,
            }
        }
        self.filtered_hooks.clone()
    }
}