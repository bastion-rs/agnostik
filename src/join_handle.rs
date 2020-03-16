use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "runtime_bastion")]
use lightproc::recoverable_handle::RecoverableHandle;

pub struct JoinHandle<R>(InnerJoinHandle<R>);

pub enum InnerJoinHandle<R> {
    #[cfg(feature = "runtime_bastion")]
    BastionHandle(RecoverableHandle<R>),
}

#[cfg(feature = "runtime_bastion")]
impl<R> Future for JoinHandle<R> {
    type Output = Option<R>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.0 {
            #[cfg(feature = "runtime_bastion")]
            InnerJoinHandle::BastionHandle(ref mut handle) => {
                Pin::new(handle).poll(cx)
            }
        }
    }
}
