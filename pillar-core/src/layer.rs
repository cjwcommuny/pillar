pub trait Layer<A, B, T> {
    fn layer<F>(self, f: F) -> T
    where
        F: FnOnce(A) -> B;
}

impl<A, B> Layer<A, B, B> for A {
    fn layer<F>(self, f: F) -> B
    where
        F: FnOnce(A) -> B,
    {
        f(self)
    }
}
// S -> (S -> S) -> S
// R -> (R -> R) -> R
// R -> (S -> S) -> R
// S -> (R -> R) -> R
// (() -> A -> B) -> ((A -> B) -> (C -> D)) -> (() -> C -> D)
