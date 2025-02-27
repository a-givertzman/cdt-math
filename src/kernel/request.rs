use futures::future::BoxFuture;
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
pub struct Request<'b, In, T> {
    op: Box<dyn AsyncFn<'b, In, T> + Send + Sync>,
}
//
//
impl<'b, In, T> Request<'b, In, T> {
    ///
    /// Returns [Request] new instance
    /// - `op` - the body of the request
    pub fn new(op: impl AsyncFn<'b, In, T> + Send + Sync + 'static) -> Self {
        let request = Self { op: Box::new(op) };
        request
    }
    ///
    /// Performs the request defined in the `op`
    pub async fn fetch(&'b self, val: In, link: Link) -> T {
        self.op.eval(val, link).await
    }
}
///
/// 
trait AsyncFn<'b, In, Out> {
    fn eval<'a>(&'a self, ctx: In, link: Link) -> BoxFuture<'b, Out> where 'a:'b;
}
//
//
impl<'b, T, F, In, Out> AsyncFn<'b, In, Out> for T
where
    T: Fn(In, Link) -> F,
    F: std::future::Future<Output = Out> + Send + 'b,
{
    fn eval<'a>(&'a self, val: In, link: Link) -> BoxFuture<'b, Out> where 'a:'b {
        Box::pin(self(val, link))
    }
}
