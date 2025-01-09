use crate::kernel::{dbgid::dbgid::DbgId, entities::{bearing::Bearing, hook::Hook}, storage::storage::Storage, str_err::str_err::StrErr};
///
/// Struct, that will be filter all bearing in storage by user hook, required static loading capacity and shank diameter
/// [documentation for bearings filter](design\docs\algorithm\part02\chapter_01_choose_hook.md)
/// - 'value' - vector of filtered bearings
pub struct BearingFilter {
    dbgid: DbgId,
    bearings: Option<Vec<Bearing>>
}
//
//
impl BearingFilter {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self { dbgid: DbgId("BearingFilter".to_string()), bearings: None }
    }
    ///
    /// Method to calculate force of gravity
    /// [documentation for force of gravity calculation filter](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'dynamic_coefficient' - value of dynamic coeffcient
    /// - 'user_load_capacity' - value of loading capacity choosen by user
    /// - 'g' - gravity constant
    fn force_of_gravity(dynamic_coefficient: f64, user_load_capacity: f64, g: f64) -> f64 {
        dynamic_coefficient*user_load_capacity*g
    }
    ///
    /// Method to filter hooks based on load capacity
    /// [documentation for bearings filter](design\docs\algorithm\part02\chapter_01_choose_hook.md)
    /// - 'storage' - [Storage] instance, where stored data base
    /// - 'user_select' - [Storage] instance, where stored user characteristics
    /// - 'user_hook' - [UserHook] instance, where stored hook, selected by user
    /// - 'dynamic_coefficient' - [DynamicCoefficient] instance, that store calculated dynamic coefficient for user characteristics
    pub fn filter(&mut self, storage: &mut Storage, user_select: &mut Storage, user_hook: Hook, dynamic_coefficient: f64) -> Result<Vec<Bearing>, StrErr> {
        let all_bearings: Vec<Bearing> = serde_json::from_value::<Vec<Bearing>>(storage.load("test.constructions.bearings")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.filter | Error {:?}",self.dbgid, err)))?;
        let force_of_gravity = BearingFilter::force_of_gravity(dynamic_coefficient, user_load_capacity, 9.81);
        let mut res_bearings: Vec<Bearing> = Vec::new();
        for bearing in all_bearings {
            if (bearing.static_load_capacity >= force_of_gravity) && (bearing.outer_diameter <= user_hook.shank_diameter) {
                res_bearings.push(bearing);
            }
        }
        self.bearings = Some(res_bearings.clone());
        Ok(res_bearings)
    }
}