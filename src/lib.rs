//! # Agnostik
//!
//! Agnostik is a layer between your application and the executor that is used to execute futures.
//! It allows you to switch between the executors smoothly and without having to change much code.
//!
//! You can use agnostik in every library that requires an executor, but wants to let the user decide
//! which executor should be used. You can also use agnostik in an application, if you plan to use multiple executors,
//! or want to switch between executors.
//!
//! ## Features
//!
//! - Run futures and wait for them to finish
//! - Spawn futures using the underlying executor
//! - Spawn blocking tasks in threads that are able to execute blocking methods
//!
//! Every feature I just said, can be used with every executor provided by agnostik, or
//! you can integrate your own executor with Agnostik.
//!
//! ## Get started
//!
//! Check the [tests](https://github.com/bastion-rs/agnostik/tree/master/tests) for simple examples.
//! 
//! If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed, you can just execute 
//! this:
//! ```text
//! cargo add agnostik
//! ```
//! 
//! otherwise, add this to your Cargo.toml file
//! ```text
//! agnostik = "0.1.0"
//! ```
//! 
//! ## Usage
//!
//! ### Switching executors
//! 
//! You can choose the executor, by using cargo features.
//! The default runtime is the `bastion-executor`.
//! To use another executor you just have to disable the default features, and choose one of the valid features.
//! Valid features are: 
//! - `runtime_bastion` (default) to use the [Bastion Executor](https://crates.io/crates/bastion-executor)
//! - `runtime_tokio` to use the [Tokio](https://tokio.rs) runtime
//! - `runtime_asyncstd` to use the [AsyncStd](https://async.rs) runtime
//! - `runtime_nostd` (coming soon) to use Agnostik in a no_std environment
//! 
//! E.g. to use the Tokio runtime, add the following line to your Cargo.toml
//! ```text
//! agnostik = { version = "0.1.0", default-features = false, features = ["runtime_tokio"]}
//! ```
//! 
//! ### Examples
//! 
//! Agnostiks API is very easy and only has a few methods to use.
//! Here's an example with the bastion-executor.
//! 
//! ```ignore
//! use agnostik::prelude::*;
//! 
//! fn main() {
//!     let runtime = Agnostik::bastion();
//! 
//!     let future = runtime.spawn(async {
//!         println!("Hello from bastions executor!");
//!     })
//!     runtime.block_on(future)
//!     
//!     let future = runtime.spawn_blocking(|| {
//!         expensive_blocking_method();
//!     })
//!     runtime.block_on(future)
//! }
//! ```
//! 
//! If you want to use another exceutor, you just have to replace the `Agnostik::bastion()`
//! method call, with the method that corresponds to your executor.
//! 
//! Use
//! - `Agnostik::bastion()` for bastion
//! - `Agnostik::async_std()` for async std
//! - `Agnostik::tokio()` for tokio. **Warning:** See "How to use tokio runtime"
//! - `Agnostik::tokio_with_runtime(runtime)` if you want to use your own `tokio::runtime::Runtime` object. **Warning:** See "How to use tokio runtime"
//! - `Agnostik::no_std()` (coming soon) to create an executor that works in a nostd environment
//! 
//! ### How to use tokio runtime
//! 
//! It's not supported to use the `tokio::main` macro together with agnostik,
//! because Agnostik requires a `Runtime` object, which is created by calling `Runtime::new()`.
//! If your are using the `tokio::main` macro, there will be a panic, because you can't create a runtime
//! inside a runtime.
//! 
//! Here's how to fix it:
//! 
//! ```ignore
//! use agnostik::prelude::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let runtime = Agnostik::tokio();
//!     
//!     let result = runtime.spawn(async_task()).await;
//! 
//!     println!("The result is {}", result)
//! }
//! ```
//! 
//! This would fail with a panic.
//! How to do it correctly:
//! 
//! ```ignore
//! use agnostik::prelude::*;
//! use tokio::runtime::Runtime;
//! 
//! fn main() {
//!     // see tokio docs for more methods to create a runtime
//!     let runtime = Runtime::new().expect("Failed to create a runtime"); // 1
//!     let runtime = Agnostik::tokio_with_runtime(runtime); // 2
//! 
//!     let result = runtime.spawn(async_task());
//!     let result = runtime.block_on(result);
//! 
//!     println!("The result is {}", result)
//! }
//! ```
//! 
//! You can replace 1 and 2 with `Agnostik::tokio()`, because this method call will
//! create a Runtime object using `Runtime::new()`.
#![cfg_attr(feature="runtime_nostd", no_std)]

mod executors;
pub mod join_handle;

use join_handle::JoinHandle;
use core::future::Future;

/// This trait represents a generic executor that can spawn a future, spawn a blocking task,
/// and wait for a future to finish.
pub trait AgnostikExecutor {
    /// Spawns an asynchronous task using the underlying executor.
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;

    /// Runs the provided closure on a thread, which can execute blocking tasks asynchronously. 
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

/// This struct doesn't have any functionality.
/// It's only use is to have a nice API to create executors
/// for different runtimes.
pub struct Agnostik;

impl Agnostik {
    #[cfg(feature = "runtime_bastion")]
    /// Returns an [AgnostikExecutor], that will use [bastion-executor] to spawn futures.
    ///
    /// [bastion-executor]: https://docs.rs/bastion-executor
    /// [AgnostikExecutor]: ./trait.AgnostikExecutor.html
    pub fn bastion() -> impl AgnostikExecutor {
        executors::BastionExecutor::new()
    }

    #[cfg(feature = "runtime_asyncstd")]
    /// Returns an [AgnostikExecutor], that will use the [AsyncStd] runtime to spawn futures.
    ///
    /// [AsyncStd]: https://docs.rs/async_std
    /// [AgnostikExecutor]: ./trait.AgnostikExecutor.html
    pub fn async_std() -> impl AgnostikExecutor {
        executors::AsyncStdExecutor::new()
    }

    #[cfg(feature = "runtime_tokio")]
    /// Returns an [AgnostikExecutor], that will use the [Tokio] runtime to spawn futures.
    ///
    /// **Attention:** This method will create a new [Runtime] object using the [Runtime::new]
    /// method and will panic if it fails to create the [Runtime] object.
    /// If you want to use your own [Runtime] object, use [tokio_with_runtime] instead.
    ///
    /// [Tokio]: https://docs.rs/tokio
    /// [Runtime]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html
    /// [Runtime::new]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html#method.new
    /// [tokio_with_runtime]: #method.tokio_with_runtime
    /// [AgnostikExecutor]: ../trait.AgnostikExecutor.html
    pub fn tokio() -> impl AgnostikExecutor {
        executors::TokioExecutor::new()
    }

    #[cfg(feature = "runtime_tokio")]
    /// Returns an [AgnostikExecutor], that will use the [Tokio] runtime to spawn futures.
    /// It will use the given [Runtime] object to spawn, and block_on futures. The spawn_blocking method
    /// will use the [tokio::task::spawn_blocking] method.
    ///
    /// [tokio::task::spawn_blocking]: https://docs.rs/tokio/0.2.13/tokio/task/fn.spawn_blocking.html
    /// [Tokio]: https://docs.rs/tokio
    /// [Runtime]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html
    /// [tokio_with_runtime]: ./fn.tokio_with_runtime.html
    /// [AgnostikExecutor]: ../trait.AgnostikExecutor.html
    pub fn tokio_with_runtime(runtime: tokio::runtime::Runtime) -> impl AgnostikExecutor {
        executors::TokioExecutor::with_runtime(runtime)
    }

    #[cfg(feature = "runtime_nostd")]
    /// Returns an [AgnostikExecutor], that will use a no_std executor from [executor] package
    ///
    /// [executor]: https://docs.rs/executor
    /// [AgnostikExecutor]: ../trait.AgnostikExecutor.html
    pub fn no_std() -> impl AgnostikExecutor {
        executors::NoStdExecutor::new()
    }
}

#[allow(unused)]
/// A prelude for the agnostik crate.
pub mod prelude {
    pub use crate::{AgnostikExecutor, Agnostik};
}
