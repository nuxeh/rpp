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

    /// Command to spawn
    //#[structopt(required_unless = "raw_command")]
    command: Vec<String>,

    /// Raw command which may contain its own argument flags
    #[structopt(conflicts_with = "command", required_unless = "command", raw = true)]
    raw_command: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    let mut rpp = Rpp::new()
        .time(opt.time)
        .peak_vm(opt.peak_vm);


    let commands = opt.command.iter()
        .chain(opt.raw_command.iter())
        .collect::<Vec<&String>>();

    if let Some(c) = commands.get(0) {
        rpp.add_command(c);
    }

    commands
        .iter()
        .skip(1)
        .for_each(|arg| rpp.add_arg(arg));

    match rpp.run() {
        Ok(_) => display_results(&opt, rpp.get_results()),
        Err(e) => eprintln!("{}", e),
    };
}

fn display_results(opt: &Opt, results: &Results) {
    eprintln!("----");

    match (opt.computer, results.get_peak_vm()) {
        (true, Some(pm)) => eprintln!("peak memory: {}", pm * 1024),
        (false, Some(pm)) => eprintln!("peak memory: {}", (pm * 1024).file_size(file_size_opts::CONVENTIONAL).unwrap()),
        _ => (),
    }

    match (opt.computer, results.get_duration(), results.get_nanos()) {
        (true, _, Some(n)) => eprintln!("time: {}", n),
        (false, Some(d), _) => eprintln!("time: {}", format_duration(d)),
        _ => (),
    }
}
