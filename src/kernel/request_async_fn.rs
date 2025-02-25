use futures::future::BoxFuture;
use crate::algorithm::context::context::Context;

use super::sync::link::Link;
///
/// Used for declarative `Rrequest` implementation
/// 
/// Example:
/// ```ignore
/// let math = AlgoSecond::new(
///     req: Request<T>::new(op: |ctx: Context| -> T {
///         let link: Link = ctx.read();
///         // Query: Some Struct comtains all neccessary info and implements `Serialize`
///         let query = QueryStruct::new();
///         // Reply: Returns `T`, implements `Deserialize`
///         link.req(query)
///     }),
///     eval: AlgFirst::new(initial),
/// )
/// ```
pub struct Request<T> {
    op: Box<dyn AsyncFn<T> + Send + Sync>,
}
//
//
impl<T> Request<T> {
    ///
    /// Returns [Request] new instance
    /// - `op` - the body of the request
    pub fn new(op: impl AsyncFn<T> + Send + Sync + 'static) -> Self {
        Self { op: Box::new(op) }
    }
    ///
    /// Performs the request defined in the `op`
    pub async fn fetch<'a>(&'a self, ctx: &'a Context, link: &'a mut Link) -> T {
        self.op.call(ctx, link).await
    }
}
///
/// 
trait AsyncFn<Out> {
    fn call<'a>(&'a self, ctx: &'a Context, link: &'a mut Link) -> BoxFuture<'a, Out>;
}
impl<'a, T, F, Out> AsyncFn<Out> for T
where
    T: Fn(&'a Context, &'a mut Link) -> F,
    F: std::future::Future<Output = Out> + Send + 'a,
{
    fn call(&'a self, ctx: &'a Context, link: &'a mut Link) -> BoxFuture<'a, Out> {
        Box::pin(self(ctx, link))
    }
}
