use crate::kernel::{dbgid::dbgid::DbgId, entities::hook::Hook, str_err::str_err::StrErr};
///
/// Struct to choose the hook from filtered, based on user characteristics
#[derive(Clone,Debug)]
pub struct UserHook {
    dbgid: DbgId,
    pub hook: Option<Hook>
}
//
//
impl UserHook {
    ///
    /// Struct Constructor
    pub fn new() -> Self {
        Self { 
            dbgid: DbgId("UserHook".to_string()),
            hook: None 
        }
    }
    ///
    /// Method to select
    /// - 'filtered_hooks' - all filtered hooks (vector of [Hook] instance)
    /// - 'user_choice' - just usize value from keyboard to make select
    pub fn select(&mut self, filtered_hooks: Vec<Hook>, user_choice: usize) -> Result<Hook,StrErr> {
        match filtered_hooks.get(user_choice) {
            Some(hook) => {
                self.hook = Some(hook.clone());
                Ok(hook.clone()) 
            },
            None => Err(StrErr(format!("{}.select | There is no hook with this index: {}", self.dbgid, user_choice))),
        }
    }
}