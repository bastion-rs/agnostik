pub use agnostik::*;

fn main() {
    let agnostik = Agnostik::new();

    let handle = agnostik.spawn(async {
        let mut i = 0;
        while i < 5 {
            println!("Counting from Asyncstd: {}", i);
            i+=1;
        }
    });

    agnostik.block_on(handle);
}
