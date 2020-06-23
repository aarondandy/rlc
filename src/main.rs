use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read};
use atty::Stream;
use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rlc")]
struct Opt {
    #[structopt(parse(from_os_str))]
    file_path: Option<PathBuf>
}

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let count = match opt.file_path {
        Some(file_path) => count_readable_stuff(File::open(file_path)?),
        None => match atty::is(Stream::Stdin) {
            false => count_readable_stuff(io::stdin()),
            true => { panic!("No input."); }
        }
    }?;

    println!("{} line(s)", count);
    Ok(())
}

fn count_readable_stuff<T: Read>(mut reader: T) -> Result<u64, std::io::Error> {
    let mut count: u64 = 0;
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut read_count: usize;
    while {
        read_count = reader.read(&mut buffer)?;
        read_count != 0
    } {
        count += buffer[..read_count].iter().filter(|b| **b == b'\n').count() as u64
    }

    Ok(count)
}
