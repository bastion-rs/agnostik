//! The bastion executor.

use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::AgnostikExecutor;
use lightproc::prelude::*;
use std::future::Future;

/// A wrapper around `bastion_executor` that implements `AgnostikExecutor`
/// and can be used to spawn and run futures using the bastion executor.
pub struct BastionExecutor;

impl BastionExecutor {
    /// Create a new `BastionExecutor` instance.
    pub const fn new() -> Self {
        BastionExecutor {}
    }
}

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
        let handle = bastion_executor::pool::spawn_blocking(async { task() }, ProcStack::default());
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
