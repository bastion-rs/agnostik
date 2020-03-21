pub mod join_handle;
mod executors;

use executors::*;
use join_handle::JoinHandle;
use std::future::Future;

pub trait AgnostikExecutor {
    /// Spawns an asynchronous task using the underlying executor.
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;

    /// runs the provided closure on a thread where blocking is allowed.
    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;

    /// Blocks until the future has finished.
    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}

#[cfg(feature = "runtime_bastion")]
pub struct Agnostik(BastionExecutor);
#[cfg(feature = "runtime_asyncstd")]
pub struct Agnostik(AsyncStdExecutor);
#[cfg(feature = "runtime_tokio")]
pub struct Agnostik(TokioExecutor);

impl Agnostik
{
    pub fn new() -> Agnostik {
        #[cfg(feature = "runtime_bastion")]
        return Agnostik(executors::BastionExecutor::new());
        #[cfg(feature = "runtime_asyncstd")]
        return Agnostik(executors::AsyncStdExecutor::new());
        #[cfg(feature = "runtime_tokio")]
        return Agnostik(executors::TokioExecutor::new());
    }
}


impl AgnostikExecutor for Agnostik
{
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static
    {
        self.0.spawn(future)
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
    {
        self.0.spawn_blocking(task)
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static
    {
        self.0.block_on(future)
    }
}
