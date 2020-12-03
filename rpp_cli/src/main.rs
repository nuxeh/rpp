use std::path::PathBuf;
use structopt::StructOpt;
use rpp::Rpp;

/// Simple profiler for processes on Linux
#[derive(StructOpt, Debug)]
#[structopt(name = "rpp")]
struct Opt {
    #[structopt(short, long)]
    time: bool,

    #[structopt(short, long)]
    memory: bool,

    /// Print non-human readable values
    #[structopt(short, long)]
    computer: bool,

    #[structopt(required = true, min_values = 1)]
    command: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let rpp = Rpp::new()
        .time(opt.time)
        .memory(opt.memory)
        .init();
}
