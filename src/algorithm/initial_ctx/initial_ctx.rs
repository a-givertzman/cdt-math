use crate::kernel::{
    dbgid::dbgid::DbgId,
    entities::{driver_type::DriverType, loading_combination::LoadingCombination},
    storage::storage::Storage,
    str_err::str_err::StrErr,
};
///
/// Struct to storage initial data
/// [documentation to initial data](design\docs\algorithm\part01\initial_data.md)
/// - 'driver_type' - [DriverType] enum, where store initial driver type
/// - 'load_comb' - [LoadingCombination] enum, where store initial loading combination
/// - 'vhmax' - value of nominal lifting speed of the mechanism
/// - 'vhcs' - value of slow lifting speed of the mechanism
#[derive(Debug, Clone)]
pub struct InitialCtx {
    // dbgid: DbgId,
    pub driver_type: DriverType,
    pub load_comb: LoadingCombination,
    pub vhmax: f64,
    pub vhcs: f64,
}
//
//
impl InitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(storage_initial_data: &mut Storage) -> Result<Self, StrErr> {
        let dbg = DbgId("InitialCtx".to_string());
        Ok(Self {
            driver_type: serde_json::from_value::<DriverType>(
                storage_initial_data.load("test.user_characteristics.driver_type")?,
            )
            .map_err(|err| StrErr(format!("{}.filter | Error {:?}", dbg, err)))?,
            load_comb: serde_json::from_value::<LoadingCombination>(
                storage_initial_data.load("test.user_characteristics.loading_combination")?,
            )
            .map_err(|err| StrErr(format!("{}.filter | Error {:?}", dbg, err)))?,
            vhmax: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.vhmax")?,
            )
            .map_err(|err| StrErr(format!("{}.filter | Error {:?}", dbg, err)))?,
            vhcs: serde_json::from_value::<f64>(
                storage_initial_data.load("test.user_characteristics.vhcs")?,
            )
            .map_err(|err| StrErr(format!("{}.filter | Error {:?}", dbg, err)))?,
            // dbgid: dbg,
        })
    }
}
