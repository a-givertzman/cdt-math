pub trait AsyncFn {
    async fn fetch(&Context, Link) Box<dyn Fn() -> T + Send + Sync>,
}