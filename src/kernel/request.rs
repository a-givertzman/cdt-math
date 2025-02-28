use futures::future::BoxFuture;
use super::sync::link::Link;
///
/// Used for declarative `Rrequest` implementation
/// 
/// Example:
/// ```ignore
/// let math = AlgoSecond::new(
///     req: Request<T>::new(async |ctx: Context, link: Link| -> T {
///         // Query: Some Struct comtains all neccessary info and implements `Serialize`
///         let query = QueryStruct::new();
///         // Reply: Returns `T`, implements `Deserialize`
///         link.req(query).await
///     }),
///     eval: AlgFirst::new(initial),
/// )
/// ```
pub struct Request<In, T> {
    op: Box<dyn AsyncFn<In, T> + Send + Sync>,
}
//
//
impl<In, T> Request<In, T> {
    ///
    /// Returns [Request] new instance
    /// - `op` - the body of the request
    pub fn new(op: impl AsyncFn<In, T> + Send + Sync + 'static) -> Self {
        let request = Self { op: Box::new(op) };
        request
    }
    ///
    /// Performs the request defined in the `op`
    pub async fn fetch(&self, val: In, link: Link) -> T {
        self.op.eval(val, link).await
    }
}
///
/// Async callback closure
pub trait AsyncFn<In, Out> {
    fn eval(&self, ctx: In, link: Link) -> BoxFuture<'_, Out>;
}
//
//
impl<T, F, In, Out> AsyncFn<In, Out> for T
where
    T: Fn(In, Link) -> F,
    F: std::future::Future<Output = Out> + Send + 'static,
{
    fn eval(&self, val: In, link: Link) -> BoxFuture<'_, Out> {
        Box::pin(self(val, link))
    }
}
