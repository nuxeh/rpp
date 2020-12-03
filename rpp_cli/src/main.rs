use std::path::PathBuf;
use structopt::StructOpt;
use rpp::{Rpp, Results};
use humantime::format_duration;
use humansize::{FileSize, file_size_opts};

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

    let mut rpp = Rpp::new()
        .time(opt.time)
        .peak_vm(opt.peak_vm);

    if let Some(c) = opt.command.get(0) {
        rpp.command(c);
    }

    opt.command
        .iter()
        .skip(1)
        .for_each(|arg| { rpp.arg(arg); });

    match rpp.run() {
        Ok(_) => display_results(&opt, rpp.get_results()),
        Err(e) => eprintln!("{}", e),
    };
}

fn display_results(opt: &Opt, results: &Results) {
    eprintln!("----");

    if let Some(pm) = results.get_peak_vm() {
        eprintln!("peak virtual memory: {}", (pm*1024).file_size(file_size_opts::CONVENTIONAL).unwrap());
    }

    match (opt.computer, results.get_duration(), results.get_nanos()) {
        (true, _, Some(n)) => eprintln!("time: {}", n),
        (false, Some(d), _) => eprintln!("time: {}", format_duration(d)),
        _ => (),
    }
}
