pub trait FnOnce<T> {
    type Output;
    fn call_once(self, arg: T) -> Self::Output;
}

pub trait Fn<T> {
    type Output;
    fn call(&self, arg: T) -> Self::Output;
}

impl<I, O, F> FnOnce<I> for F
where
    F: std::ops::FnOnce(I) -> O,
{
    type Output = O;

    fn call_once(self, arg: I) -> Self::Output {
        self(arg)
    }
}
