use std::future::Future;

use crate::ready::Already;

pub trait Service<Request> {
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a;
    type Response;

    fn call(&self, arg: Request) -> Self::Future<'_>;

    fn already(self) -> Already<Self>
    where
        Self: Sized,
    {
        Already(self)
    }
}

impl<S, Request> Service<Request> for &S
where
    S: Service<Request>,
{
    type Error = S::Error;
    type Future<'a> = S::Future<'a> where Self: 'a;
    type Response = S::Response;

    fn call(&self, arg: Request) -> Self::Future<'_> {
        (*self).call(arg)
    }
}

pub struct FnService<F>(F);

impl<F> FnService<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F, Request, Response, Fut, Error> Service<Request> for FnService<F>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Result<Response, Error>>,
{
    type Error = Error;
    type Future<'a> = Fut where Self: 'a;
    type Response = Response;

    fn call(&self, arg: Request) -> Self::Future<'_> {
        (&self.0)(arg)
    }
}
