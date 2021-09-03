mod diff;
mod diffio;
use std::fs;

fn main() {
    let diff_chars = diff::diff(
        &['_', 'a', 'b', 'c', 'n'],
        &['_', 'a', 'd', 'h', 'c', 'n'],
    );
    println!("Chars:\n{:?}", diff_chars);
    let first = fs::read("first.pack").unwrap();
    let second = fs::read("second.pack").unwrap();
    let diff = diff::diff(&first, &second);
    println!("\n\nBinaries:\n{:?}", diff);

    println!("\n\nWrite compressed file diff first->second:\n{:?}", diff);
    println!("(this operation remove useless informations)");
    diffio::write("diff.d", diff.to_owned());
    println!("\n\nGet back compressed file diff first->second:");
    println!("{:?}", diffio::read("diff.d"));
}
