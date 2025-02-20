use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::RwLock;
use crate::algorithm::context::context::Context;
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
    op: Box<dyn AsyncFn<T>>,
}
//
//
impl<T> Request<T> {
    ///
    /// Returns [Request] new instance
    /// - `op` - the body of the request
    pub fn new(op: impl AsyncFn<T> + 'static) -> Self {
        Self { op: Box::new(op) }
    }
    ///
    /// Performs the request defined in the `op`
    pub async fn fetch(&self, ctx: Arc<RwLock<Context>>) -> T {
        self.op.call(ctx).await
    }
}
///
/// 
trait AsyncFn<Out> {
    fn call(&self, ctx: Arc<RwLock<Context>>) -> BoxFuture<'static, Out>;
}
impl<T, F, Out> AsyncFn<Out> for T
where
    T: Fn(Arc<RwLock<Context>>) -> F,
    F: std::future::Future<Output = Out> + 'static + Send,
{
    fn call(&self, ctx: Arc<RwLock<Context>>) -> BoxFuture<'static, Out> {
        Box::pin(self(ctx))
    }
}