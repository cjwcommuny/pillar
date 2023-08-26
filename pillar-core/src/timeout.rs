use crate::service::ReadyService;
use futures::future::Either;
use futures::FutureExt;
use futures_concurrency::future::Race;
use std::future::Future;
use std::time::Duration;

pub struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S, Request> ReadyService<Request> for Timeout<S>
where
    S: ReadyService<Request>,
{
    type Response = S::Response;
    type Error = Error<S::Error>;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&self, arg: Request) -> Self::Future {
        let response = self.inner.call(arg);
        let sleep = tokio::time::sleep(self.timeout);
        let race = (response.map(Either::Left), sleep.map(Either::Right)).race();
        let result = race.map(|either| match either {
            Either::Left(result) => result.map_err(Error::Inner),
            Either::Right(_) => Err(Error::Elapsed),
        });
        result
    }
}

pub type TimeoutBuilder<S> = impl Fn(S) -> Timeout<S>;

impl<S> Timeout<S> {
    pub fn builder(timeout: Duration) -> TimeoutBuilder<S> {
        move |inner| Self { inner, timeout }
    }
}

pub enum Error<E> {
    Inner(E),
    Elapsed,
}
