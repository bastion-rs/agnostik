pub use agnostik::*;

#[cfg(feature = "runtime_asyncstd")]
#[test]
fn test_bastion() {
    let agnostik = Agnostik::bastion();

    let handle = agnostik.spawn(async {
        let mut i = 0;
        while i < 5 {
            println!("Counting from Bastion: {}", i);
            i+=1;
        }
    });

    agnostik.block_on(handle);
}
