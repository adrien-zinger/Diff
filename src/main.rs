mod diff;
mod diffio;
mod apply;
use std::fs;

fn human_readable(from: &str, to: &str) {
    // Human readable sample
    let a: Vec<_> = from.split("").collect();
    let b: Vec<_> = to.split("").collect();
    let diff = diff::diff(&a, &b);
    diffio::write_char("diff.d", diff);
    let diff = diffio::read("diff.d");
    let dest: Vec::<String> = apply::apply(
        a.join("").as_bytes().to_vec(),
        &diff).iter().map(|u| (*u as char).to_string()
    ).collect();
    println!("Create diff\nfrom '{}'\nto   '{}'", from, to);
    println!("Apply and obtain '{}'\n", dest.join(""));
}

fn binaries_files(from: &str, to: &str) {
    println!("Binary example");
    let first = fs::read(from).unwrap();
    let second = fs::read(to).unwrap();
    let diff = diff::diff(&first, &second);
    diffio::write("diff.d", diff.to_owned());
    println!("Binary example, read file");
    let diff = diffio::read("diff.d");
    fs::write("dest.pack", apply::apply(first, &diff)).unwrap();
}


fn main() {
    human_readable(
        "Je ne voudrais pas te faire perdre du temps",
        "Je n'ai pas envie de te faire perdre ton temps precieux"
    );
    human_readable(
        "Je n'ai pas envie de te faire perdre ton temps precieux",
        "Je ne voudrais pas te faire perdre du temps"
    );
    // Binaries
    binaries_files("first.pack", "second.pack");
    binaries_files("second.pack", "first.pack");
}
