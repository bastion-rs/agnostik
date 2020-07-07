use smol_crate as smol;

use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::{AgnostikExecutor, LocalAgnostikExecutor};
use smol::Task;
use std::future::Future;

pub struct SmolExecutor;

impl AgnostikExecutor for SmolExecutor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let task = Task::spawn(future);
        JoinHandle(InnerJoinHandle::Smol(task))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let task = Task::blocking(async { task() });
        JoinHandle(InnerJoinHandle::Smol(task))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        smol::run(future)
    }
}

impl LocalAgnostikExecutor for SmolExecutor {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let task = Task::local(future);
        JoinHandle(InnerJoinHandle::Smol(task))
    }
}
