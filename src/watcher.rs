use crate::detector::Detector;
use std::time::{Instant, Duration};
use std::thread::sleep;

pub struct Watcher {
    detector: Detector,
    interval: usize,
    verbose: bool
}

impl Watcher {
    pub fn new(detector: Detector, interval: usize, verbose: bool) -> Watcher {
        Watcher { detector, interval, verbose }
    }

    pub fn watch(&mut self) {
        loop {
            // Record start time for diagnostics
            let now = Instant::now();

            // Scan memory looking for single event upsets
            let upsets = self.detector.get_upsets();

            // Log diagnostics to stderr if verbose is set
            if self.verbose {
                eprintln!("Scanned {}b in {}ns", self.detector.get_num_bytes(), now.elapsed().as_nanos());
            }

            if !upsets.is_empty() {
                // Out put each single event upset to stdout
                for upset in upsets {
                    println!("{}", upset);
                }

                // If any single event upset occurs we need to reset the detector
                self.detector.reset();
            }

            sleep(Duration::from_secs(self.interval as u64));
        }
    }
}