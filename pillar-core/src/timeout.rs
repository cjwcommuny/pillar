use std::future::Future;
use std::time::Duration;

use futures::future::Either;
use futures::FutureExt;
use futures_concurrency::future::Race;

use crate::service::Service;

pub struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
{
    type Error = Error<S::Error>;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;
    type Response = S::Response;

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

#[cfg(test)]
mod test {
    async fn base() -> i32 {
        1
    }

    #[test]
    fn test() {
        let s1 = base;
        // let s2 = s1.layer()
    }
}
