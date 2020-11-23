pub use agnostik::prelude::*;

#[cfg(feature = "runtime_asyncstd")]
#[test]
fn test_async_std() {
    let agnostik = Agnostik::async_std();

    let handle = agnostik.spawn(async {
        let mut i = 0;
        while i < 5 {
            println!("Counting from Asyncstd: {}", i);
            i+=1;
        }
    });

    agnostik.block_on(handle);
}

#[cfg(feature = "runtime_asyncstd")]
#[test]
fn test_async_std_implicit() {
    let res = agnostik::block_on(async { agnostik::spawn(async { println!("hello world"); 1 }).await });
    assert_eq!(res, 1);
}
