use std::future::Future;

pub trait FnOnce<T> {
    type Output;
    fn call_once(self, arg: T) -> Self::Output;
}

pub trait Fn<T> {
    type Output;
    fn call(&self, arg: T) -> Self::Output;
}

pub trait AsyncFn<T> {
    type Output;
    type Future<U>: Future<Output = U>;
    fn call(&self, arg: T) -> Self::Future<Self::Output>;
}
