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
//! There's also a global executor instance that can be used to spawn futures
//! without creating and storing your own executor.
//!
//! ```ignore
//! fn main() {
//!     let future = agnostik::spawn(async { println!("Hello from bastion executor!"); 1 });
//!     let result = agnostik::block_on(future);
//!     assert_eq!(result, 1);
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
//! - `Agnostik::no_std()` (coming soon) to create an exeutor that works in a nostd environment
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
#![deny(rust_2018_idioms, clippy::pedantic, warnings, missing_docs)]

pub mod executor;
pub mod join_handle;

use join_handle::JoinHandle;
#[allow(unused)]
use once_cell::sync::Lazy;
use std::future::Future;

#[cfg(bastion)]
static EXECUTOR: Lazy<executor::BastionExecutor> = Lazy::new(|| executor::BastionExecutor);

#[cfg(async_std)]
static EXECUTOR: Lazy<executor::AsyncStdExecutor> = Lazy::new(|| executor::AsyncStdExecutor);

#[cfg(tokio)]
static EXECUTOR: Lazy<executor::TokioExecutor> = Lazy::new(|| executor::TokioExecutor::new());

#[cfg(smol)]
static EXECUTOR: Lazy<executor::SmolExecutor> = Lazy::new(|| executor::SmolExecutor);

/// and wait for a future to finish.
pub trait AgnostikExecutor {
    /// Spawns an asynchronous task using the underlying executor.
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    /// Runs the provided closure on a thread, which can execute blocking tasks asynchronously.
    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;

    /// Blocks until the future has finished.
    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

/// This trait represents an executor that is capable of spawning futures onto the same thread.
pub trait LocalAgnostikExecutor: AgnostikExecutor {
    /// Spawns a future that doesn't implement [Send].
    ///
    /// The spawned future will be executed on the same thread that called `spawn_local`.
    ///
    /// [Send]: https://doc.rust-lang.org/std/marker/trait.Send.html
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static;
}

/// This struct doesn't have any functionality.
/// It's only use is to have a nice API to create executors
/// for different runtimes.
pub struct Agnostik;

impl Agnostik {
    /// Returns an [AgnostikExecutor], that will use [bastion-executor] to spawn futures.
    ///
    /// [bastion-executor]: https://docs.rs/bastion-executor
    /// [AgnostikExecutor]: ./trait.AgnostikExecutor.html
    #[cfg(bastion)]
    pub fn bastion() -> impl AgnostikExecutor {
        executor::BastionExecutor::new()
    }

    /// Returns an [LocalAgnostikExecutor], that will use the [AsyncStd] runtime to spawn futures.
    ///
    /// [AsyncStd]: https://docs.rs/async_std
    /// [LocalAgnostikExecutor]: ./trait.LocalAgnostikExecutor.html
    #[cfg(async_std)]
    pub fn async_std() -> impl LocalAgnostikExecutor {
        executor::AsyncStdExecutor::new()
    }

    /// Returns an [LocalAgnostikExecutor], that will use the [Tokio] runtime to spawn futures.
    ///
    /// **Attention:** This method will create a new [Runtime] object using the [Runtime::new]
    /// method and will panic if it fails to create the [Runtime] object.
    /// If you want to use your own [Runtime] object, use [tokio_with_runtime] instead.
    ///
    /// [Tokio]: https://docs.rs/tokio
    /// [Runtime]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html
    /// [Runtime::new]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html#method.new
    /// [tokio_with_runtime]: #method.tokio_with_runtime
    /// [LocalAgnostikExecutor]: ../trait.LocalAgnostikExecutor.html
    #[cfg(tokio)]
    pub fn tokio() -> impl LocalAgnostikExecutor {
        executor::TokioExecutor::new()
    }

    /// Returns an [LocalAgnostikExecutor], that will use the [Tokio] runtime to spawn futures.
    /// It will use the given [Runtime] object to spawn, and block_on futures. The spawn_blocking method
    /// will use the [tokio::task::spawn_blocking] method.
    ///
    /// [tokio::task::spawn_blocking]: https://docs.rs/tokio/0.2.13/tokio/task/fn.spawn_blocking.html
    /// [Tokio]: https://docs.rs/tokio
    /// [Runtime]: https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Runtime.html
    /// [tokio_with_runtime]: ./fn.tokio_with_runtime.html
    /// [LocalAgnostikExecutor]: ../trait.LocalAgnostikExecutor.html
    #[cfg(tokio)]
    pub fn tokio_with_runtime(
        runtime: tokio_crate::runtime::Runtime,
    ) -> impl LocalAgnostikExecutor {
        executor::TokioExecutor::with_runtime(runtime)
    }

    /// Returns an [LocalAgnostikExecutor] that will use the [smol] runtime, to spawn and run futures.
    ///
    /// [smol]: https://docs.rs/smol
    /// [LocalAgnostikExecutor]: ../trait.LocalAgnostikExecutor.html
    #[cfg(smol)]
    pub fn smol() -> impl AgnostikExecutor {
        executor::SmolExecutor
    }
}

/// `spawn` will use the global executor instance, which is determined by the cargo features,
/// to spawn the given future.
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    executor().spawn(future)
}

/// `spawn_blocking` will use the global executor instance, which is determined by the cargo features,
/// to spawn the given blocking task.
pub fn spawn_blocking<F, T>(task: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    executor().spawn_blocking(task)
}

/// `block_on` will use the global executor instance, which is determined by the cargo features,
/// to block until the given future has finished.
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    executor().block_on(future)
}

/// `spawn_local` will use the global executor instance, which is determined by the cargo features,
/// to spawn a `!Send` future.
#[cfg(spawn_local)]
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    executor().spawn_local(future)
}

/// This method will set the [`tokio Runtime`] in the global executor.
///
/// [`tokio Runtime`]: https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Runtime.html
#[cfg(tokio)]
pub fn set_runtime(runtime: tokio_crate::runtime::Runtime) {
    use std::any::Any;

    let executor = executor() as &dyn Any;
    match executor.downcast_ref::<executor::TokioExecutor>() {
        Some(executor) => executor.set_runtime(runtime),
        None => unreachable!(),
    }
}

/// Returns a reference to the global executor.
#[cfg(not(local_spawn))]
pub fn executor() -> &'static impl AgnostikExecutor {
    #[cfg(enable)]
    return &*EXECUTOR;
    #[cfg(not(enable))]
    {
        struct PanicExecutor;
        impl AgnostikExecutor for PanicExecutor {
            fn spawn<F>(&self, _: F) -> JoinHandle<F::Output>
            where
                F: Future + Send + 'static,
                F::Output: Send + 'static,
            {
                panic!("no runtime feature enabled.")
            }

            fn spawn_blocking<F, T>(&self, _: F) -> JoinHandle<T>
            where
                F: FnOnce() -> T + Send + 'static,
                T: Send + 'static,
            {
                panic!("no runtime feature enabled.")
            }

            fn block_on<F>(&self, _: F) -> F::Output
            where
                F: Future + Send + 'static,
                F::Output: Send + 'static,
            {
                panic!("no runtime feature enabled.")
            }
        }
        &PanicExecutor
    }
}

/// Returns a reference to the global executor
#[cfg(local_spawn)]
pub fn executor() -> &'static impl LocalAgnostikExecutor {
    &*EXECUTOR
}

/// A prelude for the agnostik crate.
#[allow(unused)]
pub mod prelude {
    pub use crate::{block_on, spawn, spawn_blocking};
    pub use crate::{Agnostik, AgnostikExecutor, LocalAgnostikExecutor};
}
