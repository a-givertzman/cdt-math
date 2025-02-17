use crate::algorithm::entities::hook::Hook;
///
/// Struct to store user hook
#[derive(Debug, Clone, Default)]
pub struct UserHookCtx {
    /// value of user hook
    pub result: Hook,
}
