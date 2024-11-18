use crate::{algorithm::{hook_choose::hook::Hook, storage::storage::Storage, user_select::user_select::UserSelect}, kernel::{dbgid::dbgid::DbgId, entities::{mechanism_work_type::{self, MechanismWorkType}, value::Value}}};
///
/// Класс, реалилующий фильтрацию крюков по грузоподъемности
/// - 'filtered_hooks' - вектор отфильтрованных крюков (экземпляр класса [Hook])
/// [reference to filtration documentation](design\docs\algorithm\part02\chapter_01_choose_hook.md)
pub struct HookFilter{
    dbgid: DbgId,
    filtered_hooks: Vec<Hook>,
}
//
//
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
    pub fn filter(&mut self,hooks_storage: &mut Storage,hook_type: String, mechanism_work_type: MechanismWorkType, m_to_lift: f64) -> &Vec<Hook> {
        if let Some(value) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО",hook_type.trim())) {
            if let Value::NextMap(map) = value {
                for (key_iso, _) in map {
                    if let Some(vall) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/грузоподъемность/{}/",hook_type.trim(),key_iso,mechanism_work_type.to_string().trim())) {
                        if let Value::Data(datt) = vall {
                            if m_to_lift <= *datt {
                                if let Some(v) = hooks_storage.get(&format!("конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/",hook_type.trim(),key_iso
                                )) {
                                    if let Value::Data(da) = v {
                                        self.filtered_hooks.push(Hook::new(key_iso.to_string(),mechanism_work_type.to_string(),hook_type.to_string(),*datt,*da));

                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("Path not found for hook selection.");
        }

        &self.filtered_hooks

    }
}