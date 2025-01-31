///
/// Enum for structurizing types of result's
/// - 'Ok' - positive type of result
/// - 'Err' - result type with error
/// - 'None' - empty result
#[derive(Debug, Clone, PartialEq)]
pub enum CtxResult<T, E> {
    Ok(T),
    Err(E),
    None,
}
//
//
impl<T, E> CtxResult<T, E> {
    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            CtxResult::Ok(t) => t,
            CtxResult::Err(err) => {
                panic!("called `Result::unwrap()` on an `Err` value, \n\t{:?}", err)
            }
            CtxResult::None => panic!("called `Result::unwrap()` on an `None` value"),
        }
    }
}
//
//
impl<T, E> Default for CtxResult<T, E> {
    fn default() -> Self {
        Self::None
    }
}
