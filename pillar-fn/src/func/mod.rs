pub trait FnOnce<T> {
    type Output;
    fn call_once(self, arg: T) -> Self::Output;
}

pub trait Fn<T> {
    type Output;
    fn call(&self, arg: T) -> Self::Output;
}
