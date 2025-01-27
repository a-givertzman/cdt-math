use crate::{algorithm::{hoist_tackle_efficiency_coef::hoist_tackle_efficieny_coef::HoistTackleEfficienyCoeff, hoist_tackle_multiplicity::hoist_tackle_multiplicity::HoistTackleMultiplicity, rope_effort::rope_effort::RopeEffort, ropes_count::ropes_count::RopesCount}, kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr, user_setup::user_load_hand::UserLoadHandDevice}};

///
/// Struct to calculate maximum force in rope
/// [documenation to calculation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
/// - 'value' - value of maximum force in rope
pub struct MaxForce {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl MaxForce {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("MaxForce".to_string()),
            value: None,
        }
    }
    ///
    /// Method to calculate maximum force in rope
    /// [documenation to calculation](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md)
    pub fn eval(&mut self, user_select: &mut Storage, user_load_device: &mut UserLoadHandDevice) -> Result<f64,StrErr> {
        match self.value {
            Some(max_force) => Ok(max_force),
            None => {
                let user_load_capacity = serde_json::from_value::<f64>(user_select.load("test.user_characteristics.load_capacity")?).map_err(|err| StrErr(format!("{}.eval | Error {:?}",self.dbgid, err)))?;
                let (summary_weight, _payload_weight) = user_load_device.weights(user_select).map_err(|err| StrErr(format!("{}.eval | Error {:?}", self.dbgid, err)))?;
                let ropes_count_value = RopesCount::new().eval(RopeEffort::new(), user_select, user_load_device).map_err(|err| StrErr(format!("{}.eval | Error {:?}", self.dbgid, err)))?;
                let hoist_tackle_eff_coef = HoistTackleEfficienyCoeff::new().eval(HoistTackleMultiplicity::new(), user_select, user_load_device).map_err(|err| StrErr(format!("{}.eval | Error {:?}", self.dbgid, err)))?;
                let result = ((user_load_capacity+summary_weight)*9.81) / (ropes_count_value*hoist_tackle_eff_coef);
                self.value = Some(result);
                Ok(result)
            }
        }
    }
}