use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::{AgnostikExecutor, LocalAgnostikExecutor};
use std::future::Future;
use std::sync::Mutex;
use tokio_crate as tokio;

pub(crate) struct TokioExecutor(Mutex<tokio::runtime::Runtime>);

impl TokioExecutor {
    pub fn new() -> Self {
        Self::with_runtime(tokio::runtime::Runtime::new().expect("failed to create runtime"))
    }

    pub fn with_runtime(runtime: tokio::runtime::Runtime) -> Self {
        TokioExecutor(Mutex::new(runtime))
    }
}

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
