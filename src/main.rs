use std::path::PathBuf;
use structopt::StructOpt;
use single_event_upset_detector::detector::Detector;
use single_event_upset_detector::watcher::Watcher;
use single_event_upset_detector::daemonize;
use std::process::exit;

#[derive(StructOpt, Debug)]
#[structopt(name = "Single Event Upset Detector", about = "Detect single event upsets")]
struct Opts {
    #[structopt(help = "The number of bytes to use when scanning for a single event upset")]
    bytes: usize,

    #[structopt(help = "The number of seconds to wait in between scans")]
    interval: usize,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: i32,

    #[structopt(short, long, help = "Output to file when process is a daemon", parse(from_os_str))]
    file: Option<PathBuf>,

    #[structopt(short, long, help = "Run as daemon")]
    daemon: bool
}

fn main() {
    let opts: Opts = Opts::from_args();

    // If running as daemon a file must be passed
    if opts.daemon && opts.file.is_none() {
        eprintln!("file must be specified when running in the background");
        exit(1);
    }

    // Run as background process if directed
    if opts.daemon {
        daemonize(opts.file.unwrap())
    }

    // Allocate memory for the detector
    let detector = Detector::new(opts.bytes);

    // Watch the detector
    Watcher::new(detector, opts.interval, opts.verbose != 0)
        .watch();
}

