use futures::future::BoxFuture;
use crate::algorithm::context::context::Context;
use super::sync::link::Link;
///
/// Used for declarative `Rrequest` implementation
/// 
/// Example:
/// ```ignore
/// let math = AlgoSecond::new(
///     req: Request<T>::new(op: async |ctx: Context, link: &mut Link| -> T {
///         // Query: Some Struct comtains all neccessary info and implements `Serialize`
///         let query = QueryStruct::new();
///         // Reply: Returns `T`, implements `Deserialize`
///         link.req(query)
///     }),
///     eval: AlgFirst::new(initial),
/// )
/// ```
pub struct Request<'a, T> {
    op: Box<dyn AsyncFn<'a, T> + Send + Sync + 'a>,
}
//
//
impl<'a, T> Request<'a, T> {
    ///
    /// Returns [Request] new instance
    /// - `op` - the body of the request
    pub fn new(op: impl AsyncFn<'a, T> + Send + Sync + 'a) -> Self {
        Self { op: Box::new(op) }
    }
    ///
    /// Performs the request defined in the `op`
    pub async fn fetch(&self, ctx: Context, link: &mut Link) -> T {
        self.op.eval(ctx, link).await
    }
}
///
/// 
trait AsyncFn<Out> {
    fn eval<'a>(&self, ctx: Context, link: &mut Link) -> BoxFuture<'a, Out>;
}
//
//
// impl<'a, T, F, Out> AsyncFn<Out> for T
// where
//     T: Fn(Context, &mut Link) -> F,
//     F: std::future::Future<Output = Out> + Send + 'a,
// {
//     fn eval<'b>(&self, ctx: Context, link: &mut Link) -> BoxFuture<'b, Out> {
//         Box::pin(self(ctx, link))
//     }
// }
