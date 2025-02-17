use serde::{Serialize, Deserialize};

use crate::algorithm::entities::hook::Hook;
///
/// Struct to describe type of user request, that ascs user for choosing hook from filtered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChooseUserHookQuery {
    /// hooks filtered by user characteristics
    pub filtered_hooks: Vec<Hook>,
}
//
//
impl ChooseUserHookQuery {
    pub fn new(filtered_hooks: Vec<Hook>) -> Self {
        Self {
            filtered_hooks,
        }
    }
}
///
/// Reply to [ChooseUserHookQuery]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChooseUserHookReply {
    pub choosen: Hook
}
//
//
impl ChooseUserHookReply {
    pub fn new(choosen: Hook) -> Self {
        Self {
            choosen,
        }
    }
}