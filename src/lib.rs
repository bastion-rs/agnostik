pub mod join_handle;
mod executors;

use executors::*;
use join_handle::JoinHandle;
use std::future::Future;

pub trait AgnostikExecutor {
    /// Spawns an asynchronous task using the underlying executor.
    fn spawn<F, T>(self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;

    /// runs the provided closure on a thread where blocking is allowed.
    fn spawn_blocking<F, T>(self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;

    /// Blocks until the future has finished.
    fn block_on<F, T>(&mut self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}


pub struct Agnostik<K>(K) where K: AgnostikExecutor;

impl<K> Agnostik<K>
where
    K: AgnostikExecutor
{
    #[cfg(feature = "runtime_bastion")]
    pub fn new() -> Agnostik<BastionExecutor> {
       Agnostik(executors::BastionExecutor::new())
    }

    #[cfg(feature = "runtime_asyncstd")]
    pub fn new() -> Agnostik<AsyncStdExecutor> {
        Agnostik(executors::AsyncStdExecutor::new())
    }

    #[cfg(feature = "runtime_tokio")]
    pub fn new() -> Agnostik<TokioExecutor> {
        Agnostik(executors::TokioExecutor::new())
    }
}


impl<K> AgnostikExecutor for Agnostik<K>
where
    K: AgnostikExecutor
{
    fn spawn<F, T>(self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static
    {
        self.0.spawn(future)
    }

    fn spawn_blocking<F, T>(self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
    {
        self.0.spawn_blocking(task)
    }

    fn block_on<F, T>(&mut self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static
    {
        self.0.block_on(future)
    }
}
