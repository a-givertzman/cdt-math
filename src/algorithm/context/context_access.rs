use sal_sync::services::entity::error::str_err::StrErr;

use super::{context::Context, ctx_result::CtxResult};
use crate::{
    algorithm::{
        bearing_filter::bearing_filter_ctx::BearingFilterCtx, dynamic_coefficient::dynamic_coefficient_ctx::DynamicCoefficientCtx, hoist_rope_filter::hoist_rope_filter_ctx::HoistRopeFilterCtx, hoisting_tackle::hoisting_tackle_ctx::HoistingTackleCtx, hoisting_tackle_effiency_coefficient::hoist_tackle_eff_coeff_ctx::HoistTackleEffCoeffCtx, hoisting_tackle_multiplicity::hoist_tackle_multi_ctx::HoistTackleMultiCtx, hook_filter::hook_filter_ctx::HookFilterCtx, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed_ctx::LiftingSpeedCtx, load_hand_device_mass::load_hand_device_mass_ctx::LoadHandDeviceMassCtx, maximum_force::max_force_ctx::MaxForceCtx, min_break_force::min_break_force_ctx::MinBreakForceCtx, rope_count::rope_count_ctx::RopeCountCtx, rope_effort::rope_effort_ctx::RopeEffortCtx, rope_safety_factor::safety_factor_ctx::SafetyFactorCtx, select_betta_phi::select_betta_phi_ctx::SelectBetPhiCtx
    },
    kernel::user_setup::{user_bearing_ctx::UserBearingCtx, user_hoist_rope_ctx::UserHoistRopeCtx, user_hook_ctx::UserHookCtx},
};
///
/// Provides restricted write access to the [Context] members
pub trait ContextWrite<T> {
    fn write(self, value: T) -> CtxResult<Context, StrErr>;
}
///
/// Provides simple read access to the [Context] members
pub trait ContextRead<T> {
    fn read(&self) -> &T;
}
//
//
impl ContextWrite<InitialCtx> for Context {
    fn write(mut self, value: InitialCtx) -> CtxResult<Self, StrErr> {
        self.initial = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<InitialCtx> for Context {
    fn read(&self) -> &InitialCtx {
        &self.initial
    }
}
//
//
impl ContextWrite<DynamicCoefficientCtx> for Context {
    fn write(mut self, value: DynamicCoefficientCtx) -> CtxResult<Self, StrErr> {
        self.dynamic_coefficient = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<DynamicCoefficientCtx> for Context {
    fn read(&self) -> &DynamicCoefficientCtx {
        &self.dynamic_coefficient
    }
}
//
//
impl ContextWrite<LiftingSpeedCtx> for Context {
    fn write(mut self, value: LiftingSpeedCtx) -> CtxResult<Self, StrErr> {
        self.lifting_speed = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<LiftingSpeedCtx> for Context {
    fn read(&self) -> &LiftingSpeedCtx {
        &self.lifting_speed
    }
}
//
//
impl ContextWrite<SelectBetPhiCtx> for Context {
    fn write(mut self, value: SelectBetPhiCtx) -> CtxResult<Self, StrErr> {
        self.select_bet_phi = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<SelectBetPhiCtx> for Context {
    fn read(&self) -> &SelectBetPhiCtx {
        &self.select_bet_phi
    }
}
//
//
impl ContextWrite<HookFilterCtx> for Context {
    fn write(mut self, value: HookFilterCtx) -> CtxResult<Self, StrErr> {
        self.hook_filter = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<HookFilterCtx> for Context {
    fn read(&self) -> &HookFilterCtx {
        &self.hook_filter
    }
}
//
//
impl ContextWrite<UserHookCtx> for Context {
    fn write(mut self, value: UserHookCtx) -> CtxResult<Self, StrErr> {
        self.user_hook = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<UserHookCtx> for Context {
    fn read(&self) -> &UserHookCtx {
        &self.user_hook
    }
}
//
//
impl ContextWrite<BearingFilterCtx> for Context {
    fn write(mut self, value: BearingFilterCtx) -> CtxResult<Self, StrErr> {
        self.bearing_filter = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<BearingFilterCtx> for Context {
    fn read(&self) -> &BearingFilterCtx {
        &self.bearing_filter
    }
}
//
//
impl ContextWrite<UserBearingCtx> for Context {
    fn write(mut self, value: UserBearingCtx) -> CtxResult<Self, StrErr> {
        self.user_bearing = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<UserBearingCtx> for Context {
    fn read(&self) -> &UserBearingCtx {
        &self.user_bearing
    }
}
//
//
impl ContextWrite<LoadHandDeviceMassCtx> for Context {
    fn write(mut self, value: LoadHandDeviceMassCtx) -> CtxResult<Self, StrErr> {
        self.load_device_mass = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<LoadHandDeviceMassCtx> for Context {
    fn read(&self) -> &LoadHandDeviceMassCtx {
        &self.load_device_mass
    }
}
//
//
impl ContextWrite<RopeEffortCtx> for Context {
    fn write(mut self, value: RopeEffortCtx) -> CtxResult<Self, StrErr> {
        self.rope_effort = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<RopeEffortCtx> for Context {
    fn read(&self) -> &RopeEffortCtx {
        &self.rope_effort
    }
}
//
//
impl ContextWrite<RopeCountCtx> for Context {
    fn write(mut self, value: RopeCountCtx) -> CtxResult<Self, StrErr> {
        self.rope_count = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<RopeCountCtx> for Context {
    fn read(&self) -> &RopeCountCtx {
        &self.rope_count
    }
}
//
//
impl ContextWrite<HoistingTackleCtx> for Context {
    fn write(mut self, value: HoistingTackleCtx) -> CtxResult<Self, StrErr> {
        self.hoisting_tackle = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<HoistingTackleCtx> for Context {
    fn read(&self) -> &HoistingTackleCtx {
        &self.hoisting_tackle
    }
}
//
//
impl ContextWrite<HoistTackleMultiCtx> for Context {
    fn write(mut self, value: HoistTackleMultiCtx) -> CtxResult<Self, StrErr> {
        self.hoist_tackle_multi = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<HoistTackleMultiCtx> for Context {
    fn read(&self) -> &HoistTackleMultiCtx {
        &self.hoist_tackle_multi
    }
}
//
//
impl ContextWrite<HoistTackleEffCoeffCtx> for Context {
    fn write(mut self, value: HoistTackleEffCoeffCtx) -> CtxResult<Self, StrErr> {
        self.hoist_tackle_eff_coeff = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<HoistTackleEffCoeffCtx> for Context {
    fn read(&self) -> &HoistTackleEffCoeffCtx {
        &self.hoist_tackle_eff_coeff
    }
}
//
//
impl ContextWrite<MaxForceCtx> for Context {
    fn write(mut self, value: MaxForceCtx) -> CtxResult<Self, StrErr> {
        self.max_force = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<MaxForceCtx> for Context {
    fn read(&self) -> &MaxForceCtx {
        &self.max_force
    }
}
//
//
impl ContextWrite<SafetyFactorCtx> for Context {
    fn write(mut self, value: SafetyFactorCtx) -> CtxResult<Self, StrErr> {
        self.safety_factor = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<SafetyFactorCtx> for Context {
    fn read(&self) -> &SafetyFactorCtx {
        &self.safety_factor
    }
}
//
//
impl ContextWrite<MinBreakForceCtx> for Context {
    fn write(mut self, value: MinBreakForceCtx) -> CtxResult<Self, StrErr> {
        self.min_break_force = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<MinBreakForceCtx> for Context {
    fn read(&self) -> &MinBreakForceCtx {
        &self.min_break_force
    }
}
//
//
impl ContextWrite<HoistRopeFilterCtx> for Context {
    fn write(mut self, value: HoistRopeFilterCtx) -> CtxResult<Self, StrErr> {
        self.hoist_rope_filter = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<HoistRopeFilterCtx> for Context {
    fn read(&self) -> &HoistRopeFilterCtx {
        &self.hoist_rope_filter
    }
}
//
//
impl ContextWrite<UserHoistRopeCtx> for Context {
    fn write(mut self, value: UserHoistRopeCtx) -> CtxResult<Self, StrErr> {
        self.user_hoist_rope = value;
        CtxResult::Ok(self)
    }
}
impl ContextRead<UserHoistRopeCtx> for Context {
    fn read(&self) -> &UserHoistRopeCtx {
        &self.user_hoist_rope
    }
}