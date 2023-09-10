pub mod func;

use std::future::Future;

pub trait AsyncFn<T> {
    type Output;
    type Future: Future<Output = Self::Output>;
    fn call(&self, arg: T) -> Self::Future;
}

impl<F, T, U, Fut> AsyncFn<T> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = U>,
{
    type Future = Fut;
    type Output = U;

    fn call(&self, arg: T) -> Self::Future {
        self(arg)
    }
}

#[cfg(test)]
mod test {
    use std::future;

    use crate::AsyncFn;

    fn test(_f: impl AsyncFn<i32>) {}

    fn test2() {
        test(|_x| future::ready(1))
    }
}
