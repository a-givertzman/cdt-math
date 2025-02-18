use super::{context::Context, ctx_result::CtxResult};
use crate::{
    algorithm::{
        dynamic_coefficient::dynamic_coefficient_ctx::DynamicCoefficientCtx, hook_filter::hook_filter_ctx::HookFilterCtx, initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed_ctx::LiftingSpeedCtx, select_betta_phi::select_betta_phi_ctx::SelectBetPhiCtx
    },
    kernel::{link::Link, str_err::str_err::StrErr, user_setup::user_hook_ctx::UserHookCtx},
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
impl ContextWrite<Link> for Context {
    fn write(mut self, value: Link) -> CtxResult<Self, StrErr> {
        self.link = value.into();
        CtxResult::Ok(self)
    }
}
impl ContextRead<Link> for Context {
    fn read(&self) -> &Link {
        &self.link
    }
}
