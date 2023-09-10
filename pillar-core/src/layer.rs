use pillar_fn::func;

pub trait Layer<A> {
    fn layer<F, B>(self, f: F) -> B
    where
        F: func::FnOnce<A, Output = B>;
}

impl<A> Layer<A> for A {
    fn layer<F, B>(self, f: F) -> B
    where
        F: func::FnOnce<A, Output = B>,
    {
        f.call_once(self)
    }
}
// (s -> s) => (ready s -> ready s)
//
