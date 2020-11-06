pub use agnostik::prelude::*;
pub use tokio_crate as tokio;

#[cfg(feature = "runtime_tokio")]
mod tokio_tests {
    use super::*;
    #[test]
    fn test_tokio() {
        let agnostik = Agnostik::tokio();

        let handle = agnostik.spawn(async {
            let mut i = 0;
            while i < 5 {
                println!("Counting from Tokio: {}", i);
                i += 1;
            }
        });

        agnostik.block_on(handle);
    }

    #[test]
    fn test_basic_scheduler() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let rt = agnostik::Agnostik::tokio_with_runtime(rt);
        let rt = std::sync::Arc::new(rt);

        for _ in 0..100 {
            let rt = rt.clone();
            std::thread::spawn(move || {
                rt.block_on(
                    async move { tokio::time::sleep(std::time::Duration::from_secs(1)).await },
                );
            });
        }
    }
}
