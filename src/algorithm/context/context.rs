use crate::{algorithm::lifting_speed::lifting_speed_ctx::LiftingSpeedCtx, kernel::initial_ctx::initial_ctx::InitialCtx};
///
/// # Calculation context
/// - Provides read/write access to initial
/// - R/W access to the isoleted data of each step of computations
#[derive(Debug, Clone)]
pub struct Context {
    pub initial: InitialCtx,
    pub lifting_speed: LiftingSpeedCtx,
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
        }
    }
}
