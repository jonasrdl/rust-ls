use structopt::StructOpt;
use std::path::PathBuf;

mod output_format;
mod entry_processing;

#[derive(Debug, StructOpt)]
#[structopt(name = "ls", about = "A replacement for the ls command")]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,

    #[structopt(short, long)]
    all: bool,

    #[structopt(short, long)]
    long: bool,
}

fn main() {
    let opt = Opt::from_args();
    let path = opt.path.unwrap_or_else(|| PathBuf::from("."));

    if let Err(err) = entry_processing::list_files(&path, opt.all, opt.long) {
        eprintln!("Error: {}", err);
    }
}