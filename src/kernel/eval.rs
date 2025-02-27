use futures::future::BoxFuture;
///
/// Trate defines common evaluation function for calculations classes
pub trait Eval<In, Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval<'a>(&'a mut self, val: In) -> BoxFuture<'a, Out>;
}

