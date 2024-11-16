use crate::{algorithm::{hook_choose::hook::Hook, storage::storage::Storage, user_select::user_select::UserSelect}, kernel::{dbgid::dbgid::DbgId, entities::value::Value}};
///
/// Класс, реалилующий фильтрацию крюков по грузоподъемности
/// - 'filtered_hooks' - вектор отфильтрованных крюков
/// - 'm_to_lift' - масса на крюке
/// - 'mechanism_work_type' - режим работы механизма согласно ГОСТ 34017-2016
/// - 'hook_type' - тип крюка
pub struct HookFilter{
    dbgid: DbgId,
    filtered_hooks: Vec<Hook>,
    m_to_lift: f64,
    mechanism_work_type: String,
    hook_type: String,
}
//
//
//
impl HookFilter{
    ///
    /// Конструктор класса HookFilter
    pub fn new(user_select: &UserSelect) ->Self{
        Self {  dbgid: DbgId(format!("HookFilter")),
                filtered_hooks: Vec::new(),
                m_to_lift: user_select.m_to_lift, 
                mechanism_work_type: user_select.mechanism_work_type.clone(),
                hook_type: user_select.hook_type.clone(),
             }
    }
    ///
    /// Метод фильтрации БД крюков по грузоподъемности
    /// - 'hooks_storage' - хранилище крюков экземпляр класса [Storage]
    pub fn filter(
        &mut self,
        hooks_storage: &mut Storage,
    ) -> &Vec<Hook> {
        if let Some(value) = hooks_storage.get(&format!(
            "конструкции/крюки/тип крюка/{}/ИСО",
            self.hook_type.trim()
        )) {
            if let Value::NextMap(map) = value {
                for (key_iso, _) in map {
                    if let Some(vall) = hooks_storage.get(&format!(
                        "конструкции/крюки/тип крюка/{}/ИСО/{}/грузоподъемность/{}/",
                        self.hook_type.trim(),
                        key_iso,
                        self.mechanism_work_type.trim()
                    )) {
                        if let Value::Data(datt) = vall {
                            if self.m_to_lift <= *datt {
                                if let Some(v) = hooks_storage.get(&format!(
                                    "конструкции/крюки/тип крюка/{}/ИСО/{}/диаметр хвостовика/",
                                    self.hook_type.trim(),
                                    key_iso
                                )) {
                                    if let Value::Data(da) = v {
                                        self.filtered_hooks.push(Hook::new(key_iso.to_string(),self.mechanism_work_type.to_string(),self.hook_type.to_string(),*datt,*da));

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