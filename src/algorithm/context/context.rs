use crate::algorithm::{
    dynamic_coefficient::dynamic_coefficient_ctx::DynamicCoefficientCtx,
    hook_filter::hook_filter_ctx::HookFilterCtx, initial_ctx::initial_ctx::InitialCtx,
    lifting_speed::lifting_speed_ctx::LiftingSpeedCtx,
    select_betta_phi::select_betta_phi_ctx::SelectBetPhiCtx,
};
///
/// # Calculation context
/// - Provides read/write access to initial
/// - R/W access to the isoleted data of each step of computations
#[derive(Debug, Clone)]
pub struct Context {
    /// where store [initial data](design\docs\algorithm\part01\initial_data.md)
    pub initial: InitialCtx,
    /// result of calculation [steady-state-lifting-speed](design/docs/algorithm/part02/chapter_01_choose_hook.md)
    pub(super) lifting_speed: LiftingSpeedCtx,
    /// result of calculation [ϕ2(phi) and β2(betta) coefficients](design/docs/algorithm/part02/chapter_01_choose_hook.md)
    pub(super) select_bet_phi: SelectBetPhiCtx,
    /// result of calculation [dynamic coefficient](design/docs/algorithm/part02/chapter_01_choose_hook.md)
    pub(super) dynamic_coefficient: DynamicCoefficientCtx,
    /// result of [filtering hooks](design/docs/algorithm/part02/chapter_01_choose_hook.md)
    pub(super) hook_filter: HookFilterCtx,
}
//
//
impl Context {
    ///
    /// Struct constructor
    /// - 'initial' - [InitialCtx] instance, where store initial data
    pub fn new(initial: InitialCtx) -> Self {
        Self {
            initial,
            lifting_speed: LiftingSpeedCtx::default(),
            select_bet_phi: SelectBetPhiCtx::default(),
            dynamic_coefficient: DynamicCoefficientCtx::default(),
            hook_filter: HookFilterCtx::default(),
        }
    }
}
