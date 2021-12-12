extern crate test;
use test::Bencher;

use super::*;
use std::fs;

fn human_readable(from: &str, to: &str) {
  // Human readable sample
  let a: Vec<_> = from.split("").collect();
  let b: Vec<_> = to.split("").collect();
  let diff = diff::diff(&a, &b);
  diffio::_write_char("diff.d", diff);
  let diff = diffio::read("diff.d");
  let _: Vec::<String> = apply::apply(
      a.join("").as_bytes().to_vec(),
      &diff).iter().map(|u| (*u as char).to_string()
  ).collect();
  //println!("Create diff\nfrom '{}'\nto   '{}'", from, to);
  //println!("Apply and obtain '{}'\n", dest.join(""));
}

fn binaries_files(from: &str, to: &str) {
  //println!("Binary example");
  let first = fs::read(from).unwrap();
  let second = fs::read(to).unwrap();
  let diff = diff::diff(&first, &second);
  diffio::write("diff.d", diff);
  //println!("Binary example, read file");
  let diff = diffio::read("diff.d");
  fs::write("dest.pack", apply::apply(first, &diff)).unwrap();
}

#[test]
fn run_tests() {
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

#[bench]
fn benchmark(b: &mut Bencher) {
  let from = "first.pack";
  let to = "second.pack";
  let first = fs::read(from).unwrap();
  let second = fs::read(to).unwrap();
  b.iter(move || {
    diff::diff(&first, &second);
  });
}

#[bench]
fn benchmark_text(b: &mut Bencher) {
  b.iter(move || {
    human_readable(
      "Je n'ai pas envie de te faire perdre ton temps precieux",
      "Je ne voudrais pas te faire perdre du temps"
    );
  });
}

#[bench]
fn dev_test(b: &mut Bencher) {
  let from: Vec<u8> = "Je n'ai pas envie de te faire perdre ton temps precieux".as_bytes().into();
  let to: Vec<u8> = "Je ne voudrais pas te faire perdre du temps".as_bytes().into();
  let mut res= 0;
  let chunks_from = from.chunks_exact(4);
  println!("size {}", chunks_from.len());
  b.iter( || {
    let chunks_from = from.chunks_exact(2);
    let chunks_to = to.chunks_exact(2);
    res = 0;
    chunks_from.zip(chunks_to.into_iter()).for_each(|(a, b)| {
      if a == b {
        res += 1;
      }
    });
  });
  println!("equals {}", res);
}
