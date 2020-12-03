use std::path::PathBuf;
use structopt::StructOpt;
use rpp::Rpp;

/// Simple profiler for processes on Linux
#[derive(StructOpt, Debug)]
#[structopt(name = "rpp")]
struct Opt {
    /// Execution time
    #[structopt(short, long)]
    time: bool,

    /// Peak memory usage
    #[structopt(short = "m", long)]
    peak_vm: bool,

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
        .peak_vm(opt.peak_vm)
        .init();
}
