use std::time::{Instant, Duration};
use std::thread::sleep;
use structopt::StructOpt;
use single_event_upset_detector::detector::Detector;

#[derive(StructOpt, Debug)]
#[structopt(name = "Single Event Upset Detector", about = "Detect single event upsets")]
struct Opts {
    #[structopt(help = "The number of bytes to use when scanning for a single event upset")]
    bytes: usize,

    #[structopt(help = "The number of seconds to wait in between scans")]
    interval: usize,

    #[structopt(short = "-v", long = "--verbose", parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::from_args();

    // Allocate memory for the detector
    let mut detector = Detector::new(opts.bytes);

    loop {
        // Record start time for diagnostics
        let now = Instant::now();

        // Scan memory looking for single event upsets
        let upsets = detector.get_upsets();

        // Log diagnostics to stderr if verbose is set
        if opts.verbose > 0 {
            eprintln!("Scanned {}b in {}ns", opts.bytes, now.elapsed().as_nanos());
        }

        if !upsets.is_empty() {
            // Out put each single event upset to stdout
            for upset in upsets {
                println!("{}", upset);
            }

            // If any single event upset occurs we need to reset the detector
            detector.reset();
        }

        sleep(Duration::from_secs(opts.interval as u64));
    }
}

