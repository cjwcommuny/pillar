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
    type Output = U;
    type Future = Fut;

    fn call(&self, arg: T) -> Self::Future {
        self(arg)
    }
}

#[cfg(test)]
mod test {
    use crate::AsyncFn;
    use std::future;

    fn test(f: impl AsyncFn<i32>) {}

    fn test2() {
        test(|x| future::ready(1))
    }
}
