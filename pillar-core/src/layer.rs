pub trait Layer<A> {
    fn layer<F, B>(self, f: F) -> B
    where
        F: FnOnce(A) -> B;
}

impl<A> Layer<A> for A {
    fn layer<F, B>(self, f: F) -> B
    where
        F: FnOnce(A) -> B,
    {
        f(self)
    }
}
