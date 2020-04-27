//! This module contains all no_std related implementations.

use pin_utils::pin_mut;

use crate::AgnostikExecutor;
use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::noop_waker::noop_waker;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use alloc::boxed::Box;

pub(crate) struct NoStdJoinHandle<R> {
    pub(crate) inner: Pin<Box<dyn Future<Output = R>>>,
}

impl<R> NoStdJoinHandle<R> {
    pub(crate) fn new(f: Box<dyn Future<Output = R>>) -> Self {
        Self { inner: f.into() }
    }
}

pub(crate) struct NoStdExecutor;

impl AgnostikExecutor for NoStdExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let future = NoStdJoinHandle::new(Box::new(future));
        JoinHandle(InnerJoinHandle::NoStd(future))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(async { task() })
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let waker = noop_waker();
        pin_mut!(future);
        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(val) => break val,
                Poll::Pending => {},
            }
        }
    }
}
