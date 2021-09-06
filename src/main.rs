mod diff;
mod diffio;
mod apply;
use std::fs;

fn main() {
    // Human readable sample
    let a: Vec<_> = "Je ne voudrais pas te faire perdre du temps".split("").collect();
    let b: Vec<_> = "Je n'ai pas envie de te faire perdre ton temps precieux".split("").collect();
    println!("Example with strings\n{}\n{}", a.join(""), b.join(""));
    let diff = diff::diff(&a, &b);
    diffio::debug(&diff);
    println!("\n\nWrite compressed diff");
    diffio::write_char("diff.d", diff);
    diffio::debug_u8_to_char(&diffio::read("diff.d"));

    // Binaries
    println!("\n\nBinary example");
    let first = fs::read("first.pack").unwrap();
    let second = fs::read("second.pack").unwrap();
    let diff = diff::diff(&first, &second);
    diffio::debug(&diff);
    diffio::write("diff.d", diff.to_owned());
    println!("\n\nBinary example, read file");
    diffio::debug(&diffio::read("diff.d"));
}
