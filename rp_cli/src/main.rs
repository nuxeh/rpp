use std::path::PathBuf;
use structopt::StructOpt;

/// Simple profiler for processes
#[derive(StructOpt, Debug)]
#[structopt(name = "rp")]
struct Opt {
    #[structopt(short, long)]
    time: bool,

    #[structopt(short, long)]
    memory: bool,

    #[structopt(short, long, parse(from_os_str))]
    command: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
