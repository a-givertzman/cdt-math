use crate::infrostructure::client::{choose_user_hoist::ChooseUserHoistQuery, choose_user_hoist_characteristics::ChooseHoistCharactetisticsQuery};

use super::{
    change_hoisting_tackle::ChangeHoistingTackleQuery,
    choose_hoisting_rope::ChooseHoistingRopeQuery, choose_user_bearing::ChooseUserBearingQuery,
    choose_user_hook::ChooseUserHookQuery,
};
use serde::{Deserialize, Serialize};
///
/// List of all possible requests in Client-Server interface
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Query {
    ///
    /// Request for choosing hook from filtered hooks
    ChooseUserHook(ChooseUserHookQuery),
    ///
    /// Request for choosing bearing from filtered bearings
    ChooseUserBearing(ChooseUserBearingQuery),
    ///
    /// Request for choosing hoisting rope
    ChooseHoistingRope(ChooseHoistingRopeQuery),
    ///
    /// Request for changing hoisting tackle
    ChangeHoistingTackle(ChangeHoistingTackleQuery),
    ///
    /// Request for choosing hoist characteristics
    ChooseHoistCharactetistics(ChooseHoistCharactetisticsQuery),
    /// Request for choosing hoist from filtered
    ChooseUserHoist(ChooseUserHoistQuery),
}
