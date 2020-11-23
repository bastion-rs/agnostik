use smol_crate as smol;

use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::AgnostikExecutor;
use std::future::Future;

/// A wrapper around the `smol` crate which implements `AgnostikExecutor` and
/// `LocalAgnostikExecutor`.
pub struct SmolExecutor;

impl SmolExecutor {
    /// Create a new `SmolExecutor`.
    pub const fn new() -> Self {
        Self
    }
}

impl AgnostikExecutor for SmolExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let task = smol::spawn(future);
        JoinHandle(InnerJoinHandle::Smol(task))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let task = smol::spawn(smol::unblock(|| task()));
        JoinHandle(InnerJoinHandle::Smol(task))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        smol::block_on(future)
    }
}
