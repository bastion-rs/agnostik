use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(not(any(
    feature = "runtime_tokio",
    feature = "runtime_asyncstd",
    feature = "runtime_bastion"
)))]
use futures::future::RemoteHandle;

#[cfg(feature = "runtime_asyncstd")]
use async_std::task::JoinHandle as AsyncStdHandle;
#[cfg(feature = "runtime_bastion")]
use lightproc::recoverable_handle::RecoverableHandle;
#[cfg(feature = "runtime_tokio")]
use tokio::task::JoinHandle as TokioHandle;

pub struct JoinHandle<R>(pub(crate) InnerJoinHandle<R>);

pub(crate) enum InnerJoinHandle<R> {
    #[cfg(feature = "runtime_bastion")]
    Bastion(RecoverableHandle<R>),
    #[cfg(feature = "runtime_asyncstd")]
    AsyncStd(AsyncStdHandle<R>),
    #[cfg(feature = "runtime_tokio")]
    Tokio(TokioHandle<R>),
    #[cfg(not(any(
        feature = "runtime_tokio",
        feature = "runtime_asyncstd",
        feature = "runtime_bastion"
    )))]
    RemoteHandle(RemoteHandle<R>),
}

impl<R> Future for JoinHandle<R>
where
    R: 'static + Send,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
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
            #[cfg(not(any(
                feature = "runtime_tokio",
                feature = "runtime_asyncstd",
                feature = "runtime_bastion"
            )))]
            InnerJoinHandle::RemoteHandle(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
