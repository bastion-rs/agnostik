//! Generic join handle type.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "runtime_asyncstd")]
use async_std_crate::task::JoinHandle as AsyncStdHandle;
#[cfg(feature = "runtime_bastion")]
use lightproc::recoverable_handle::RecoverableHandle;
#[cfg(feature = "runtime_tokio")]
use tokio_crate::task::JoinHandle as TokioHandle;

/// A handle that awaits the result of a task.
///
/// This handle will be returned by a method that spawns an
/// asynchronous task. It then can be used to await the tasks termination.
///
/// **Note:** If you are using the bastion or tokio executor,
/// agnostik will panic if the task failed to execute.
pub struct JoinHandle<R>(pub(crate) InnerJoinHandle<R>);

pub(crate) enum InnerJoinHandle<R> {
    #[cfg(feature = "runtime_bastion")]
    Bastion(RecoverableHandle<R>),
    #[cfg(feature = "runtime_asyncstd")]
    AsyncStd(AsyncStdHandle<R>),
    #[cfg(feature = "runtime_tokio")]
    Tokio(TokioHandle<R>),
    #[cfg(feature = "runtime_smol")]
    Smol(smol_crate::Task<R>),
}

impl<R> Future for JoinHandle<R>
where
    R: 'static + Send,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0 {
            #[cfg(feature = "runtime_bastion")]
            InnerJoinHandle::Bastion(ref mut handle) => Pin::new(handle)
                .poll(cx)
                .map(|val| val.expect("task failed to execute")),
            #[cfg(feature = "runtime_asyncstd")]
            InnerJoinHandle::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx),
            #[cfg(feature = "runtime_tokio")]
            InnerJoinHandle::Tokio(ref mut handle) => Pin::new(handle)
                .poll(cx)
                .map(|val| val.expect("task failed to execute")),
            #[cfg(feature = "runtime_smol")]
            InnerJoinHandle::Smol(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
