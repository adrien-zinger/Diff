mod diff;
mod diffio;
mod apply;
use std::path::Path;
use std::fs;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    binary: Option<bool>,
    #[structopt(short, long)]
    apply: Option<bool>,
    #[structopt(parse(from_os_str))]
    pub from: PathBuf,
    #[structopt(parse(from_os_str))]
    pub to: PathBuf,
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,
}

fn apply(diff: &Path, file: &Path, is_binary: bool, output: &Path) {
    let file = fs::read(file).unwrap();
    let diff = diffio::read(&diff.to_string_lossy());
    if is_binary {
        fs::write(output, apply::apply(file, &diff)).unwrap();
    } else {
        let res: Vec::<String> = apply::apply(file, &diff)
            .iter()
            .map(|u| (*u as char).to_string())
            .collect();
        fs::write(output, &res.join("")).unwrap();
    }
}

fn diff(from: &Path, to: &Path, output: &Path) {
    let first = fs::read(from).unwrap();
    let second = fs::read(to).unwrap();
    diffio::write(&output.to_string_lossy(), diff::diff(&first, &second));
}

fn main() {
    let opt: Opt = Opt::from_args();
    let is_apply = opt.apply.unwrap_or(false);
    if is_apply {
        let is_binary = opt.binary.unwrap_or(true);
        apply(&opt.from, &opt.to, is_binary, &opt.output);
    } else {
        diff(&opt.from, &opt.to, &opt.output);
    }
}

#[cfg(test)]
mod tests;