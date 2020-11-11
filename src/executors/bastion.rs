//! The bastion executor.

use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::AgnostikExecutor;
use bastion_executor::blocking::*;
use lightproc::prelude::*;
use std::future::Future;

pub struct BastionExecutor;

impl BastionExecutor {
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
