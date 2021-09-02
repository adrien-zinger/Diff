mod diff;
use std::fs;

fn main() {
    let diff_chars = diff::diff(
        &['_', 'a', 'b', 'c', 'n'],
        &['_', 'a', 'd', 'h', 'c', 'n'],
    );
    println!("Chars:\n{:?}", diff_chars);
    //println!("u8\n{}", diff::diff(&[0_u8, 4, 5, 7, 8], &[0u8, 4, 6, 8, 9, 10]));
    let first = fs::read("first.pack").unwrap();
    let second = fs::read("second.pack").unwrap();
    println!("Binaries:\n{:?}", diff::diff(&first, &second));
}
