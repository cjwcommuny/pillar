use std::future;
use std::future::Future;

pub trait Ready<T> {
    type Error;

    type TryFuture<'a, U>: Future<Output = Result<&'a U, Self::Error>>
    where
        Self: 'a,
        U: 'a;

    fn ready(&self) -> Self::TryFuture<'_, T>;
}

impl<T> Ready<T> for T {
    type Error = !;
    type TryFuture<'a, U> = future::Ready<Result<&'a U, Self::Error>> where Self: 'a, U: 'a;

    fn ready(&self) -> Self::TryFuture<'_, T> {
        future::ready(Ok(self))
    }
}

pub struct MapReady<R, F> {
    inner: R,
    f: F,
}

type FF<A, B, E> = impl FnOnce(Result<A, E>) -> Result<B, E>;

impl<A, B, R, F> Ready<B> for MapReady<R, F>
where
    R: Ready<A>,
    F: FnOnce(&A) -> &B,
{
    type Error = R::Error;
    type TryFuture<'a, U> = futures::future::Map<R::TryFuture<'a, A>, FF<&'a A, &'a B, Self::Error>>
        where
            Self: 'a,
            U: 'a;

    fn ready(&self) -> Self::TryFuture<'_, B> {
        todo!()
    }
}

// impl<A, B, R, F, O> Ready<B> for MapReady<R, F>
// where
//     R: Ready<A>,
//     F: FnOnce(R::Output<'_, A>) -> O,
// {
//     type Error = R::Error;
//     type Output<'a, U> = O where Self: 'a, U: 'a;
//     type Future<'a, U>
//     where
//         Self: 'a,
//         U: 'a;
//
//     fn ready(&self) -> Self::Future<'_, B> {
//         todo!()
//     }
// }

// pub trait ReadyExt<T>: Ready<T> {
//     fn map<U, F>(self, f: F) -> MapReady<T, Self, F>
//     where
//         F: FnOnce(T) -> U,
//     {
//         todo!()
//     }
// }

// impl<A, B, R1, R2> Layer<A, B, R2> for R1
// where
//     R1: Ready<A>,
//     R2: Ready<B>,
// {
//     fn layer<F>(self, f: F) -> R2
//     where
//         F: FnOnce(A) -> B,
//     {
//         todo!()
//     }
// }
