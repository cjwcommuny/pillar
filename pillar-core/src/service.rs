use std::future::Future;

pub trait ReadyService<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn call(&self, arg: Request) -> Self::Future;
}

pub trait Service<Request> {
    type Error;
    type ReadyFuture: Future<Output = Result<Self::ReadyService, Self::Error>>;
    type ReadyService: ReadyService<Request, Error = Self::Error>;

    fn ready_service() -> Self::ReadyFuture;

    // TODO: &self vs self
    fn layer<F, S, R>(self, f: F) -> S
    where
        F: Fn(Self) -> S,
        S: Service<R>,
        Self: Sized,
    {
        f(self)
    }
}

// (...) -> service -> service
