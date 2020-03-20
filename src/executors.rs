use join_handle::{InnerJoinHandle, JoinHandle};
use std::future::Future;
use crate::AgnostikExecutor;

#[cfg(feature = "runtime_asyncstd")]
pub struct AsyncStdExecutor;

#[cfg(feature = "runtime_asyncstd")]
impl AsyncStdExecutor {
    pub fn new() -> Self {
        AsyncStdExecutor {}
    }
}

#[cfg(feature = "runtime_asyncstd")]
impl AgnostikExecutor for AsyncStdExecutor {
    fn spawn<F, T>(self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn(future);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn spawn_blocking<F, T>(self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn block_on<F, T>(self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::block_on(future)
    }
}

#[cfg(feature = "runtime_tokio")]
pub struct TokioExecutor(tokio::runtime::Runtime);

#[cfg(feature = "runtime_tokio")]
impl TokioExecutor {
    pub fn new() -> Self {
        TokioExecutor(tokio::runtime::Runtime::new().expect("failed to create runtime"))
    }

    pub fn with_runtime(runtime: tokio::runtime::Runtime) -> Self {
        TokioExecutor(runtime)
    }
}

#[cfg(feature = "runtime_tokio")]
impl AgnostikExecutor for TokioExecutor {
    fn spawn<F, T>(self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.0.spawn(future);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn spawn_blocking<F, T>(self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn block_on<F, T>(&mut self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.0.block_on(future)
    }
}
