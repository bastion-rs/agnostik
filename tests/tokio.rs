pub use agnostik::*;

#[cfg(feature = "runtime_tokio")]
#[test]
fn test_tokio() {
    let agnostik = Agnostik::tokio();

    let handle = agnostik.spawn(async {
        let mut i = 0;
        while i < 5 {
            println!("Counting from Tokio: {}", i);
            i+=1;
        }
    });

    agnostik.block_on(handle);
}
