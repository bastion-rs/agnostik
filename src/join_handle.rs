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
#[cfg(feature = "runtime_tokio")]
use tokio::task::JoinHandle as TokioHandle;

pub struct JoinHandle<R>(InnerJoinHandle<R>);

enum InnerJoinHandle<R> {
    #[cfg(feature = "runtime_bastion")]
    Bastion(RecoverableHandle<R>),
    #[cfg(feature = "runtime_asyncstd")]
    AsyncStd(AsyncStdHandle<Option<R>>),
    #[cfg(feature = "runtime_tokio")]
    Tokio(TokioHandle<R>),
    RemoteHandle(RemoteHandle<Option<R>>),
}

#[cfg(feature = "runtime_bastion")]
impl<R> Future for JoinHandle<R>
where
    R: 'static + Send,
{
    type Output = Option<R>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.0 {
            #[cfg(feature = "runtime_bastion")]
            InnerJoinHandle::Bastion(ref mut handle) => Pin::new(handle).poll(cx),
            #[cfg(feature = "runtime_asyncstd")]
            InnerJoinHandle::AsyncStd(ref mut handle) => Pin::new(handle).poll(cx),
            #[cfg(feature = "runtime_tokio")]
            InnerJoinHandle::Tokio(ref mut handle) => match Pin::new(handle).poll(cx) {
                // NOTE: Just panicing if Err is returned is probably not the best solution
                Poll::Ready(val) => Poll::Ready(Some(val.expect("task failed to execute"))),
                Poll::Pending => Poll::Pending,
            },
            InnerJoinHandle::RemoteHandle(ref mut handle) => Pin::new(handle).poll(cx),
        }
    }
}
