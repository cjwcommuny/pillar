use std::future::Future;

trait ReadyService<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn call(&self, arg: Request) -> Self::Future;
}

trait Service<Request> {
    type Error;
    type ReadyFuture: Future<Output = Result<Self::ReadyService, Self::Error>>;
    type ReadyService: ReadyService<Request, Error = Self::Error>;

    fn ready_service() -> Self::ReadyFuture;

    // TODO: &self vs self
    fn layer<F, S, R>(&self, f: F) -> S
    where
        F: for<'a> Fn(&'a Self) -> S,
        S: Service<R>,
    {
        f(self)
    }
}

// (...) -> service -> service
