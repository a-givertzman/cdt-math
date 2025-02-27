use futures::future::BoxFuture;
///
/// Trate defines common evaluation function for calculations classes
pub trait Eval<'a, Inp, Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval(&'a mut self, val: Inp) -> BoxFuture<'a, Out>;
}
