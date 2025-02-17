use serde::{Serialize, Deserialize};

use crate::algorithm::entities::hook::Hook;
///
/// Struct to describe type of user request, that ascs user for choosing hook from filtered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChooseUserHookQuery {
    /// hooks filtered by user characteristics
    pub filtered_hooks: Vec<Hook>,
    #[serde(skip_serializing_if = "testing")]
    pub testing: bool,
}
//
//
impl ChooseUserHookQuery {
    pub fn new(filtered_hooks: Vec<Hook>) -> Self {
        Self {
            filtered_hooks,
            testing: false,
        }
    }
    ///
    /// Used for testing
    pub fn test(filtered_hooks: Vec<Hook>) -> Self {
        Self {
            filtered_hooks,
            testing: true,
        }
    }
}
///
/// Used for `skip_serializing_if`
fn testing(v: &bool) -> bool {
    !v
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