use std::future;

use futures::{TryFuture, TryFutureExt};

pub trait Ready {
    type Output<'a>
    where
        Self: 'a;

    type Error;

    type TryFuture<'a>: TryFuture<Ok = Self::Output<'a>, Error = Self::Error>
    where
        Self: 'a;

    fn ready(&self) -> Self::TryFuture<'_>;
}

pub struct Already<T>(pub(crate) T);

impl<T> Ready for Already<T> {
    type Output<'a> = &'a T where Self: 'a;
    type Error = !;
    type TryFuture<'a> = future::Ready<Result<&'a T, Self::Error>> where Self: 'a;

    fn ready(&self) -> Self::TryFuture<'_> {
        future::ready(Ok(&self.0))
    }
}

pub struct MapReady<R, F> {
    inner: R,
    f: F,
}

impl<'c, R, F, B> Ready for MapReady<R, F>
where
    R: Ready,
    F: for<'b> Fn(R::Output<'b>) -> B + 'static,
{
    type Output<'a> = B  where Self: 'a;
    type Error = R::Error;
    type TryFuture<'a> = futures::future::MapOk<R::TryFuture<'a>, &'a F> where Self: 'a;

    fn ready(&self) -> Self::TryFuture<'_> {
        self.inner.ready().map_ok(&self.f)
    }
}

pub struct ReadyFn<F>(F);

impl<F, O, E, Fut> Ready for ReadyFn<F>
where
    F: Fn() -> Fut,
    Fut: TryFuture<Ok = O, Error = E>,
{
    type Output<'a> = O where Self: 'a;
    type Error = E;
    type TryFuture<'a> = Fut where Self: 'a;

    fn ready(&self) -> Self::TryFuture<'_> {
        self.0()
    }
}
