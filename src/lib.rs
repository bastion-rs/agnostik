mod executors;
pub mod join_handle;

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

pub struct Agnostik;

impl Agnostik {
    #[cfg(feature = "runtime_bastion")]
    pub fn bastion() -> impl AgnostikExecutor {
        executors::BastionExecutor::new()
    }

    #[cfg(feature = "runtime_asyncstd")]
    pub fn async_std() -> impl AgnostikExecutor {
        executors::AsyncStdExecutor::new()
    }

    #[cfg(feature = "runtime_tokio")]
    pub fn tokio() -> impl AgnostikExecutor {
        executors::TokioExecutor::new()
    }

    #[cfg(feature = "runtime_tokio")]
    pub fn tokio_with_runtime(runtime: tokio::runtime::Runtime) -> impl AgnostikExecutor {
        executors::TokioExecutor::with_runtime(runtime)
    }
}
