#![no_std]

pub use agnostik::prelude::*;

#[cfg(feature = "runtime_nostd")]
#[test]
fn test_nostd() {
    let agnostik = Agnostik::no_std();

    let handle = agnostik.spawn(async {
        let mut i = 0;
        while i < 5 {
            i+=1;
        }
    });

    agnostik.block_on(handle);
}
