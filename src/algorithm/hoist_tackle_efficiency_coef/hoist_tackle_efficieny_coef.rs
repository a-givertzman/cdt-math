use crate::{algorithm::{hoist_tackle_multiplicity::hoist_tackle_multiplicity::HoistTackleMultiplicity, hoisting_tackle::hoisting_tackle::HoistingTackle, ropes_count::ropes_count::RopesCount}, kernel::{dbgid::dbgid::DbgId, storage::storage::Storage, str_err::str_err::StrErr, user_setup::user_load_hand::UserLoadHandDevice}};
///
/// Struct to calculate hoisting tackle efficiency coefficient
/// - 'value' - value of hoisting tackle efficieny coefficient
pub struct HoistTackleEfficienyCoeff {
    dbgid: DbgId,
    value: Option<f64>
}
//
//
impl HoistTackleEfficienyCoeff {
    ///
    /// Struct constructor
    pub fn new() -> Self {
        Self {
            dbgid: DbgId("HoistTackleEfficienyCoeff".to_string()),
            value: None
        }
    }
    ///
    /// Method to calculate hoisting tackle efficiency coefficient
    /// - 'hoist_tackle_multiplicity' - [HoistTackleMultiplicity] instance, where store value
    /// - 'user_select' - [Storage] instance, where user characteristics are store
    /// - 'user_load_device' - [UserLoadHandDevice] instance, where summary info about user loading handing device store
    pub fn eval(&mut self,mut hoist_tackle_multiplicity: HoistTackleMultiplicity, user_select: &mut Storage,mut user_load_device: UserLoadHandDevice) -> Result<f64,StrErr> {
        match self.value {
            Some(hoist_tackle_eff_coef) => Ok(hoist_tackle_eff_coef),
            None => {
                let num_defl_blocks = serde_json::from_value::<f64>(user_select.load("test.constructions.number_of_deflection_blocks_for_hoisting_tackle")?).map_err(|err| StrErr(format!("{}.eval | Error: {:?}",self.dbgid,err)))?;
                let hoist_tackle_multi_value = hoist_tackle_multiplicity.eval(&mut RopesCount::new(), &mut HoistingTackle::new(), user_select, &mut user_load_device).expect(&format!("{}.eval | Error to calculate hoisting tackle multiplicity",self.dbgid));
                let result = 0.985_f64.powf(num_defl_blocks) * ((1.0-0.98_f64.powf(hoist_tackle_multi_value))/((1.0-0.98)*hoist_tackle_multi_value));
                self.value = Some(result);
                Ok(result)
            }
        }
    }
} 