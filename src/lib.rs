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

tokio! {
    extern crate tokio;

    use tokio::task::JoinHandle;

    impl Agnostik {
        pub fn spawn<T, F>(future: F) -> JoinHandle<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
        {
            tokio::spawn(future)
        }
    }
}

asyncstd! {
    extern crate async_std;

    use async_std::task::{self, JoinHandle};

    impl Agnostik {
        pub fn spawn<T, F>(future: F) -> JoinHandle<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
        {
            task::spawn(future)
        }
    }
}
