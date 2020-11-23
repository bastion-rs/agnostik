//! Generic join handle type.

use std::{
    convert::Infallible,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(async_std)]
use async_std_crate::task::JoinHandle as AsyncStdHandle;
#[cfg(bastion)]
use lightproc::recoverable_handle::RecoverableHandle;
#[cfg(tokio)]
use tokio_crate::task::JoinHandle as TokioHandle;

/// A handle that awaits the result of a task.
///
/// This handle will be returned by a method that spawns an
/// asynchronous task. It then can be used to await the tasks termination.
///
/// **Note:** If you are using the bastion or tokio executor,
/// agnostik will panic if the task failed to execute.
#[pin_project::pin_project]
pub struct JoinHandle<R>(#[pin] pub InnerJoinHandle<R>);

/// Inner join handle representation to hold variants
/// of the executors
#[pin_project::pin_project(project = JoinHandleProj)]
pub enum InnerJoinHandle<R> {
    /// The `JoinHandle` which is used for the bastion executor.
    #[cfg(bastion)]
    Bastion(#[pin] RecoverableHandle<R>),
    /// The `JoinHandle` which is used for the async_std runtime.
    #[cfg(async_std)]
    AsyncStd(#[pin] AsyncStdHandle<R>),
    /// The `JoinHandle` which is used for the tokio runtime.
    #[cfg(tokio)]
    Tokio(#[pin] TokioHandle<R>),
    /// The `JoinHandle` which is used for the smol runtime.
    #[cfg(smol)]
    Smol(#[pin] smol_crate::Task<R>),

    /// Private element that can not be constructed.
    #[doc(hidden)]
    __Private(Infallible, PhantomData<R>),
}

impl<R> Future for JoinHandle<R>
where
    R: 'static + Send,
{
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().0.poll(cx)
    }
}

impl<R> Future for InnerJoinHandle<R>
where
    R: 'static + Send,
{
    type Output = R;

    #[allow(unused_mut, unused_variables)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            #[cfg(bastion)]
            JoinHandleProj::Bastion(handle) => handle
                .poll(cx)
                .map(|val| val.expect("task failed to execute")),
            #[cfg(async_std)]
            JoinHandleProj::AsyncStd(handle) => handle.poll(cx),
            #[cfg(tokio)]
            JoinHandleProj::Tokio(handle) => handle
                .poll(cx)
                .map(|val| val.expect("task failed to execute")),
            #[cfg(smol)]
            JoinHandleProj::Smol(handle) => handle.poll(cx),
            JoinHandleProj::__Private(_, _) => unreachable!(),
        }
    }
}
