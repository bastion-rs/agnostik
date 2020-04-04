pub use agnostik::prelude::*;

#[cfg(feature = "runtime_nostd")]
#[test]
fn test_no_std() {
    let agnostik = Agnostik::no_std();
    agnostik.block_on(async {
        let mut i = 0;
        while i < 5 {
            println!("Counting from no_std: {}", i);
            i+=1;
        }
    });
}
