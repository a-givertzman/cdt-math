use sal_sync::services::entity::{
    error::str_err::StrErr, 
    name::Name
};
use crate::algorithm::entities::{
    crane_work_area_type::CraneWorkArea, 
    hoist_group::HoistGroup, 
    winding_type::WindingType
};

use super::safety_factor_ctx::SafetyFactorCtx;
///
/// Selection step: [safety factor coefficient](design\docs\algorithm\part02\chapter_04_choose_hoist_rope.md) from [table](references\GOST_33710-2015.pdf)
pub struct SelectSafetyCoeff {
    dbg: Name,
    winding_type: WindingType,
    marking_of_fire_explosion_hazardous_operating_environment: bool,
    crane_work_area: CraneWorkArea,
    mechanism_work_type: HoistGroup,
}
//
//
impl SelectSafetyCoeff {
    ///
    /// New instance [SelectSafetyCoeff]
    /// - 'winding_type' - parsing parameter [WindingType]
    /// - `mark_fire_exp_env` - parsing parameter, which marks [fire/explosive environment](design\docs\algorithm\part01\initial_data.md)
    /// - `crane_work_area` - parsing parameter [CraneWorkArea]
    /// - `mechanism_work_type` - parsing parameter [HoistGroup]
    pub fn new(
        parent: impl Into<String>, 
        winding_type: WindingType,
        marking_of_fire_explosion_hazardous_operating_environment: bool,
        crane_work_area: CraneWorkArea,
        mechanism_work_type: HoistGroup,
    ) -> Self {
        Self {
            dbg: Name::new(parent, "SelectSafetyCoeff"),
            winding_type,
            marking_of_fire_explosion_hazardous_operating_environment,
            crane_work_area,
            mechanism_work_type,
        }
    }
    ///
    /// Returns coefficient from [table choice](references\GOST_33710-2015.pdf)
    pub fn eval(&self) -> Result<SafetyFactorCtx, StrErr> {
        let result = if self.marking_of_fire_explosion_hazardous_operating_environment {
            match self.winding_type {
                WindingType::SingleLayer => 4.5,
                WindingType::MultiLayer => 4.5,
            }
        } else {
            match self.crane_work_area {
                CraneWorkArea::Default => {
                    match self.winding_type {
                        WindingType::SingleLayer => {
                            // Value of safety factor for M1-M8 type of [HoistGroup] and single [WindingType].
                            match self.mechanism_work_type {
                                HoistGroup::M1 => 3.15,
                                HoistGroup::M2 => 3.35,
                                HoistGroup::M3 => 3.55,
                                HoistGroup::M4 => 4.0,
                                HoistGroup::M5 => 4.5,
                                HoistGroup::M6 => 5.6,
                                HoistGroup::M7 => 7.1,
                                HoistGroup::M8 => 9.0,
                            }
                        }
                        WindingType::MultiLayer => {
                            // Value of safety factor for M1-M8 type of [HoistGroup] and multi [WindingType].
                            match self.mechanism_work_type {
                                HoistGroup::M1 => 3.55,
                                HoistGroup::M2 => 3.55,
                                HoistGroup::M3 => 3.55,
                                HoistGroup::M4 => 4.0,
                                HoistGroup::M5 => 4.5,
                                HoistGroup::M6 => 5.6,
                                HoistGroup::M7 => {
                                    return Err(StrErr(format!(
                                        "{}.eval | For multilayer winding for mode `M7` the rope safety factor is unknown",
                                        self.dbg
                                    )))
                                }
                                HoistGroup::M8 => {
                                    return Err(StrErr(format!(
                                        "{}.eval | For multilayer winding for mode `M8` the rope safety factor is unknown",
                                        self.dbg
                                    )))
                                }
                            }
                        }
                    }
                }
                _ => {
                    match self.winding_type {
                        WindingType::SingleLayer => 4.5,
                        WindingType::MultiLayer => 4.5,
                    }
                }
            }
        };
        Ok(SafetyFactorCtx { result })
    }
}