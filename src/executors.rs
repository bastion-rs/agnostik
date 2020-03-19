use join_handle::{InnerJoinHandle, JoinHandle};
use std::future::Future;
use crate::AgnostikExecutor;

#[cfg(feature = "runtime_asyncstd")]
pub(crate) struct AsyncStdExecutor;

#[cfg(feature = "runtime_asyncstd")]
impl AsyncStdExecutor {
    pub const fn new() -> Self {
        AsyncStdExecutor {}
    }
}

#[cfg(feature = "runtime_asyncstd")]
impl AgnostikExecutor for AsyncStdExecutor {
    fn spawn<F, T>(future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn(future);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn spawn_blocking<F, T>(task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn block_on<F, T>(future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::block_on(future)
    }
}

#[cfg(feature = "runtime_tokio")]
pub(crate) struct TokioExecutor;

#[cfg(feature = "runtime_tokio")]
impl TokioExecutor {
    pub const fn new() -> Self {
        TokioExecutor {}
    }
}

#[cfg(feature = "runtime_tokio")]
impl AgnostikExecutor for TokioExecutor {
    fn spawn<F, T>(future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = tokio::task::spawn(future);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn spawn_blocking<F, T>(task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn block_on<F, T>(future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        unimplemented!("Tokio's agnostik executor doesnt support running futures via this method")
    }
}
