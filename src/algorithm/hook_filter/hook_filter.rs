use crate::kernel::{dbgid::dbgid::DbgId, entities::hook::Hook, storage::storage::Storage, str_err::str_err::StrErr};
///
/// Struct, that will be filter all hooks in storage by user characteristics, required lifting capacity
/// [documentation to filter by load capacity](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - vector of filtered hooks
#[derive(Debug, Clone)]
pub struct HookFilter {
    dbgid: DbgId,
    value: Option<Vec<Hook>>
}
//
//
impl HookFilter { 
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { 
            dbgid: DbgId("HookFilter".to_string()),
            value: None
        }
    }
    ///
    /// Method to filter hooks based on load capacity
    /// [documentation for hooks filter](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'user_select' - [Storage] instance, where user characteristics are stored
    /// - 'storage' - [Storage] instance, where stored data base
    pub fn filter(&mut self, mut user_select: Storage, mut storage: Storage) -> Result<Vec<Hook>,StrErr> {
        let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        let user_mechanism_work_type = serde_json::from_value::<String>(user_select.load("test.user_characteristics.mechanism_work_type")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        let all_hooks: Vec<Hook> = serde_json::from_value::<Vec<Hook>>(storage.load("test.constructions.hooks")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        let mut res_hooks: Vec<Hook> = Vec::new();
        for hook in all_hooks.iter() {
            match user_mechanism_work_type.as_str() {
                "M4" | "M5" | "M6" => if hook.load_capacity_m46 <= user_load_capacity { res_hooks.push(hook.clone()) },
                "M1" | "M2" | "M3" => if hook.load_capacity_m13 <= user_load_capacity { res_hooks.push(hook.clone()) },
                "M7" | "M8" => if hook.load_capacity_m78 <= user_load_capacity { res_hooks.push(hook.clone()) },
                _ => log::error!("{}.filter | Value can not be empty", self.dbgid)
            }
        }
        self.value = Some(res_hooks.clone());
        Ok(res_hooks)
    }
}