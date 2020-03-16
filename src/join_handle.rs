use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::future::RemoteHandle;

#[cfg(feature = "runtime_asyncstd")]
use async_std::task::JoinHandle as AsyncStdHandle;
#[cfg(feature = "runtime_bastion")]
use lightproc::recoverable_handle::RecoverableHandle;

pub struct JoinHandle<R>(InnerJoinHandle<R>);

pub enum InnerJoinHandle<R> {
    #[cfg(feature = "runtime_bastion")]
    Bastion(RecoverableHandle<R>),
    #[cfg(feature = "runtime_asyncstd")]
    AsyncStd(AsyncStdHandle<Option<R>>),
    RemoteHandle(RemoteHandle<R>),
}

#[cfg(feature = "runtime_bastion")]
impl<R> Future for JoinHandle<R> {
    type Output = Option<R>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.0 {
            #[cfg(feature = "runtime_bastion")]
            InnerJoinHandle::Bastion(ref mut handle) => Pin::new(handle).poll(cx),
            #[cfg(feature = "runtime_asyncstd")]
            InnerJoinHandle::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx),
            InnerJoinHandle::RemoteHandle(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
