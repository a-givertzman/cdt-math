use serde::{
    Serialize, 
    Deserialize
};
use crate::algorithm::entities::hoist::hoist::Hoist;
///
/// User request | Asks user for choose [Hook] from filtered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChooseUserHoistQuery {
    /// vector of hooks filtered by user characteristics
    pub variants: Vec<Hoist>,
    #[serde(skip_serializing_if = "testing")]
    pub testing: bool,
}
//
//
impl ChooseUserHoistQuery {
    ///
    /// New instance [ChooseUserHoistQuery]
    pub fn new(variants: Vec<Hoist>) -> Self {
        Self {
            variants,
            testing: false,
        }
    }
    ///
    /// New instance [ChooseUserHoistQuery] for testing
    pub fn test(variants: Vec<Hoist>) -> Self {
        Self {
            variants,
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
/// Reply to [ChooseUserHoistQuery]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChooseUserHoistReply {
    pub answer: Hoist
}
//
//
impl ChooseUserHoistReply {
    ///
    /// New instance [ChooseUserHoistReply]
    pub fn new(answer: Hoist) -> Self {
        Self {
            answer,
        }
    }
}