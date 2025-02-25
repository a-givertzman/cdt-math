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
    pub async fn fetch(&'a self, ctx: Context, link: &'a mut Link) -> T {
        self.op.eval(ctx, link).await
    }
}


// pub trait AsyncFn<'a, Out> {
//     ///
//     /// Pervorms a calculation
//     /// - Returns [Out] contains results inside
//     async fn eval(&'a self, ctx: Context, link: &'a mut Link) -> Out;
// }

///
/// 
trait AsyncFn<'a, Out> {
    fn eval(&'a self, ctx: Context, link: &'a mut Link) -> BoxFuture<'a, Out>;
}
impl<'a, T, F, Out> AsyncFn<'a, Out> for T
where
    T: Fn(Context, &'a mut Link) -> F,
    F: std::future::Future<Output = Out> + Send + 'a,
{
    fn eval(&'a self, ctx: Context, link: &'a mut Link) -> BoxFuture<'a, Out> {
        Box::pin(self(ctx, link))
    }
}
