use std::future::Future;

pub trait Service<Request> {
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    type Response;

    fn call(&self, arg: Request) -> Self::Future;
}

impl<S, Request> Service<Request> for &S
where
    S: Service<Request>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    fn call(&self, arg: Request) -> Self::Future {
        (*self).call(arg)
    }
}
