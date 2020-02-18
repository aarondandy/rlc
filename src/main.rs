use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read};
use atty::Stream;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rlc")]
struct Opt {
    #[structopt(parse(from_os_str))]
    file_path: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let count = match (opt.file_path, atty::is(Stream::Stdin)) {
        (Some(file_path), true) => count_lines_in_file(file_path),
        (None, false) => count_lines_from_pipe(),
        (Some(_), false) => { panic!("Ambiguous input."); },
        (None, true) => { panic!("No input."); }
    }?;

    println!("{} line(s)", count);
    Ok(())
}

fn count_lines_in_file(file_path: PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let source = File::open(file_path)?;
    let bytes = source.bytes()
        .filter_map(Result::ok)
        .filter(|b| *b == b'\n');
    let mut count: u64 = 0;
    for _ in bytes {
        count += 1;
    }
    Ok(count)
}

fn count_lines_from_pipe() -> Result<u64, Box<dyn std::error::Error>> {
    let source = io::stdin();
    let bytes = source.lock().bytes()
        .filter_map(Result::ok)
        .filter(|b| *b == b'\n');
    let mut count: u64 = 0;
    for _ in bytes {
        count += 1;
    }
    Ok(count)
}
