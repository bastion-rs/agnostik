//! This module contains some utility functions that are used
//! in agnostik and can be used by other projects as well.

use std::{
    boxed::Box,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub fn map_future<F, M, I, O>(future: F, mapper: M) -> impl Future<Output = O>
where
    F: Future<Output = I> + 'static,
    M: Fn(I) -> O,
{
    let future = Box::new(future);
    MappedFuture {
        future: Pin::from(future),
        mapper,
        result: None,
    }
}

pub struct MappedFuture<F, I, O> {
    future: Pin<Box<dyn Future<Output = I>>>,
    mapper: F,
    result: Option<O>,
}

impl<F, I, O> Future for MappedFuture<F, I, O>
where
    F: Fn(I) -> O,
{
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.future.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(val) => Poll::Ready((self.mapper)(val)),
        }
    }
}
