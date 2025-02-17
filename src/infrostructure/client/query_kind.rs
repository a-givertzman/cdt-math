use serde::{Deserialize, Serialize};

use super::{choose_user_bearing::ChooseUserBearingQuery, choose_user_hook::ChooseUserHookQuery};

pub enum Event {
    Query(QueryKind),
    Inf(InfKind),
    Diag(Diag),
}
///
/// List of all possible requests in Client-Server interface
#[derive(Debug, Serialize, Deserialize)]
pub enum QueryKind {
    ///
    /// Request for choosing hook from filtered hooks
    ChooseUserHook(ChooseUserHookQuery),
    ///
    /// Request for choosing bearing from filtered bearings
    ChooseUserBearing(ChooseUserBearingQuery),
    ///
    /// Request for choosing hoisting rope
    ChooseHoistingRope,
    ///
    /// Request for changing hoisting tackle
    ChangeHoistingTackle,
}
//
//
// impl FromStr for QueryKind {
//     type Err = StrErr;
//     //
//     //
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "choose-user-hook" => {
//                 Ok(QueryKind::ChooseUserHook(ChooseUserHookQuery))
//             }
//             "choose-user-bearing" => {
//                 Ok(QueryKind::ChooseUserBearing)
//             }
//             "choose-hoisting-rope" => {
//                 Ok(QueryKind::ChooseHoistingRope)
//             }
//             "change-hoisting-tackle" => {
//                 Ok(QueryKind::ChangeHoistingTackle)
//             }
//             _ => Err(StrErr(
//                 format!("{}.run | Unknown kind of query: {}", std::any::type_name::<Self>(), s),
//             )),
//         }
//     }
// }