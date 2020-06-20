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
        Some(file_path) => count_lines_in_file(file_path),
        None => match atty::is(Stream::Stdin) {
            false => count_lines_from_pipe(),
            true => { panic!("No input."); }
        }
    }?;

    println!("{} line(s)", count);
    Ok(())
}

fn count_lines_in_file(file_path: PathBuf) -> Result<u64, std::io::Error> {
    let source = File::open(file_path)?;
    count_readable_stuff(Box::new(source))
}

fn count_lines_from_pipe() -> Result<u64, std::io::Error> {
    count_readable_stuff(Box::new(io::stdin()))
}

fn count_readable_stuff(mut ready_thing: Box<dyn Read>) -> Result<u64, std::io::Error> {
    let mut count: u64 = 0;
    let mut buffer: [u8; 4096] = [0; 4096];

    loop {
        let read_count = ready_thing.read(&mut buffer[..])?;
        if read_count == 0 {
            break;
        }

        count += count_breaks(&buffer[..read_count]);
    }

    Ok(count)
}

fn count_breaks(slice: &[u8]) -> u64 {
    slice.into_iter().filter(|b| **b == b'\n').count() as u64
}
