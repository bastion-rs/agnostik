use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::AgnostikExecutor;
#[cfg(feature = "runtime_tokio")]
use crate::LocalAgnostikExecutor;

#[cfg(not(feature = "runtime_nostd"))]
use std::future::Future;

#[cfg(feature = "runtime_asyncstd")]
pub(crate) struct AsyncStdExecutor;

#[cfg(feature = "runtime_asyncstd")]
impl AsyncStdExecutor {
    pub fn new() -> Self {
        AsyncStdExecutor {}
    }
}

#[cfg(feature = "runtime_asyncstd")]
impl AgnostikExecutor for AsyncStdExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let handle = async_std::task::spawn(future);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        async_std::task::block_on(future)
    }
}

#[cfg(feature = "runtime_tokio")]
use std::sync::Mutex;

#[cfg(feature = "runtime_tokio")]
pub(crate) struct TokioExecutor(Mutex<tokio::runtime::Runtime>);

#[cfg(feature = "runtime_tokio")]
impl TokioExecutor {
    pub fn new() -> Self {
        Self::with_runtime(tokio::runtime::Runtime::new().expect("failed to create runtime"))
    }

    pub fn with_runtime(runtime: tokio::runtime::Runtime) -> Self {
        TokioExecutor(Mutex::new(runtime))
    }
}

#[cfg(feature = "runtime_tokio")]
impl AgnostikExecutor for TokioExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let handle = self.0.lock().unwrap().spawn(future);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.lock().unwrap().block_on(future)
    }
}

#[cfg(feature = "runtime_tokio")]
impl LocalAgnostikExecutor for TokioExecutor {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let handle = tokio::task::spawn_local(future);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }
}

#[cfg(feature = "runtime_bastion")]
pub(crate) struct BastionExecutor;

#[cfg(feature = "runtime_bastion")]
impl BastionExecutor {
    pub fn new() -> Self {
        BastionExecutor {}
    }
}

#[cfg(feature = "runtime_bastion")]
use bastion_executor::prelude::*;
#[cfg(feature = "runtime_bastion")]
use lightproc::prelude::*;

#[cfg(feature = "runtime_bastion")]
impl AgnostikExecutor for BastionExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let handle = bastion_executor::pool::spawn(future, ProcStack::default());
        JoinHandle(InnerJoinHandle::Bastion(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = spawn_blocking(async { task() }, ProcStack::default());
        JoinHandle(InnerJoinHandle::Bastion(handle))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        bastion_executor::run::run(future, ProcStack::default())
    }
}

#[cfg(feature = "runtime_nostd")]
use crate::join_handle::NoStdJoinHandle;
#[cfg(feature = "runtime_nostd")]
use core::task::{Waker, Context, Poll};
#[cfg(feature = "runtime_nostd")]
use core::pin::Pin;
#[cfg(feature = "runtime_nostd")]
use core::future::Future;

#[cfg(feature = "runtime_nostd")]
pub(crate) struct NoStdExecutor {
    waker: Waker,
}

#[cfg(feature = "runtime_nostd")]
impl NoStdExecutor {
    pub fn new() -> Self {
        Self { waker: crate::noop_waker::noop_waker() }
    }
}

#[cfg(feature = "runtime_nostd")]
impl AgnostikExecutor for NoStdExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let handle = NoStdJoinHandle::new(future);
        JoinHandle(InnerJoinHandle::NoStd(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let future = async { task() };
        let handle = NoStdJoinHandle::new(future);
        JoinHandle(InnerJoinHandle::NoStd(handle))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let future: Pin<&mut dyn Future> = Pin::new(future);
        let cx = &mut Context::from_waker(self.waker);
        loop {
            match future.poll(cx) {
                Poll::Ready(val) => break val,
                Poll::Pending => cx.wake(),
            }
        }
    }
}
