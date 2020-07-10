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

    let reader: Box<dyn Read> = match opt.file_path {
        Some(file_path) => Box::new(File::open(file_path)?),
        None => match atty::is(Stream::Stdin) {
            false => Box::new(io::stdin()),
            true => Err(failure::err_msg("No input"))?
        }
    };

    let count = count_readable_stuff(reader)?;

    println!("{} line(s)", count);
    Ok(())
}

fn count_readable_stuff(mut reader: Box<dyn Read>) -> Result<u64, std::io::Error> {
    let mut count: u64 = 0;
    let mut buffer: [u8; 32*1024] = [0u8; 32*1024];
    loop {
        let read_count = reader.read(&mut buffer)?;
        if read_count == 0 {
            break;
        }

        count += buffer[..read_count].into_iter().filter(|&&b| b == b'\n').count() as u64
    }

    Ok(count)
}
