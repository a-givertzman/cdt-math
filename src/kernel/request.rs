use crate::algorithm::context::context::Context;
///
/// Struct to store request
pub struct Request<T> {
    op: Box<dyn Fn(Context) -> T>,
}
//
//
impl<T> Request<T> {
    ///
    /// Struct constructor
    pub fn new(op: impl Fn(Context) -> T + 'static) -> Self {
        Self { op: Box::new(op) }
    }
    pub fn execute(&self, ctx: Context) -> T {
        (self.op)(ctx)
    }
}