///
/// Enum for structurizing types of result's
/// - 'Ok' - positive type of result
/// - 'Err' - result type with error
/// - 'None' - empty result
#[derive(Debug, Clone)]
pub enum CtxResult<T, E> {
    Ok(T),
    Err(E),
    None,
}
//
//
impl<T, E> Default for CtxResult<T, E> {
    fn default() -> Self {
        Self::None
    }
}