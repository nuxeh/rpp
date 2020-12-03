use std::path::PathBuf;
use structopt::StructOpt;
use rpp::{Rpp, Results};
use humantime::format_duration;

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
    if let Some(pm) = results.get_peak_vm() {
        println!("peak virtual memory: {}", pm);
    }

    match (opt.computer, results.get_duration(), results.get_nanos()) {
        (true, _, Some(n)) => println!("time: {}", n),
        (false, Some(d), _) => println!("time: {}", format_duration(d)),
        _ => (),
    }
}
