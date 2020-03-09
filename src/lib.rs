#[macro_use]
mod macros;

use std::future::Future;

pub struct Agnostik();

bastion! {
    extern crate lightproc;
    extern crate bastion_executor;

    use lightproc::{ proc_stack::ProcStack, recoverable_handle::RecoverableHandle};
    use bastion_executor::pool;

    impl Agnostik {
        pub fn spawn<T, F>(future: F) -> RecoverableHandle<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
        {
            let proc_stack = ProcStack::default();
            pool::spawn(future, proc_stack)
        }
    }
}
