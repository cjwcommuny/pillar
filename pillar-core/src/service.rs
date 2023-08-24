use pillar_fn::AsyncFn;
use std::future::Future;

trait Service<Request> {
    type Response;
    type Error;
    type ReadyFuture<U>: Future<Output = U>;
    type ReadyService: AsyncFn<Request, Output = Result<Self::Response, Self::Error>>;

    fn ready_service() -> Self::ReadyFuture<Result<Self::ReadyService, Self::Error>>;
}
