use std::future::Future;
use std::marker::PhantomData;

use futures::TryFutureExt;
use pillar_fn::func;
use tokio::sync::{mpsc, oneshot};

use crate::service::Service;

pub struct Buffer<Request, Response, Error> {
    sender: mpsc::Sender<Message<Request, Response, Error>>,
}

impl<Request, Response, Error> Buffer<Request, Response, Error> {
    pub fn builder<S>(bound: usize) -> BufferBuilder<Request> {
        BufferBuilder {
            bound,
            mark: PhantomData,
        }
    }
}

pub struct Message<Request, Response, Error> {
    request: Request,
    response_sender: oneshot::Sender<Result<Response, Error>>,
}

pub struct BufferBuilder<Request> {
    bound: usize,
    mark: PhantomData<fn(Request)>,
}

impl<S, Request> func::FnOnce<S> for BufferBuilder<Request>
where
    S: Service<Request> + Send + Sync + 'static,
    for<'a> S::Future<'a>: Send,
    S::Error: Send,
    S::Response: Send,
    Request: Send + 'static,
{
    type Output = Buffer<Request, S::Response, S::Error>;

    fn call_once(self, service: S) -> Self::Output {
        let (sender, mut receiver) = mpsc::channel(self.bound);
        tokio::spawn(async move {
            while let Some(Message {
                request,
                response_sender,
            }) = receiver.recv().await
            {
                let result = service.call(request).await;
                response_sender.send(result).ok();
            }
        });
        Buffer { sender }
    }
}

impl<Request, Response, Err> Service<Request> for Buffer<Request, Response, Err> {
    type Error = Error<Err>;
    type Future<'a> = impl Future<Output = Result<Self::Response, Self::Error>> + 'a where Self: 'a;
    type Response = Response;

    fn call(&self, request: Request) -> Self::Future<'_> {
        let (response_sender, receiver) = oneshot::channel();
        let message = Message {
            request,
            response_sender,
        };
        let output = self
            .sender
            .send(message)
            .map_err(|_| Error::Buffer)
            .and_then(|_| {
                receiver
                    .map_err(|_| Error::Buffer)
                    .and_then(|result| std::future::ready(result.map_err(Error::Inner)))
            });
        output
    }
}

pub enum Error<E> {
    Inner(E),
    Buffer,
}
