use crate::algorithm::{
    initial_ctx::initial_ctx::InitialCtx, lifting_speed::lifting_speed_ctx::LiftingSpeedCtx,
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
    /// where store info about result of algorithm [LiftingSpeedCtx]
    pub lifting_speed: LiftingSpeedCtx,
    /// where store info about result of algorithm [SelectBetPhiCtx]
    pub bet_phi: SelectBetPhiCtx,
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
            bet_phi: SelectBetPhiCtx::default(),
        }
    }
}