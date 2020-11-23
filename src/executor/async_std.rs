//! The async std executor

use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::{AgnostikExecutor, LocalAgnostikExecutor};
use async_std_crate as async_std;
use std::future::Future;

/// A wrapper around the `async-std` crate which implements `AgnostikExecutor` and
/// `LocalAgnostikExecutor`.
pub struct AsyncStdExecutor;

impl AsyncStdExecutor {
    /// Create a new `AsyncStdExecutor`.
    pub const fn new() -> Self {
        AsyncStdExecutor {}
    }
}

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

impl LocalAgnostikExecutor for AsyncStdExecutor {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let handle = async_std::task::spawn_local(future);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }
}
