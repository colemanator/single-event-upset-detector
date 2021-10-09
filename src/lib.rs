#![feature(portable_simd)]

use std::path::PathBuf;
use std::fs::OpenOptions;
use daemonize::Daemonize;

pub mod detector;
pub mod watcher;

pub fn daemonize(path: PathBuf) {
    let daemonize = Daemonize::new()
        .stdout(OpenOptions::new().create(true).append(true).open(path.as_path()).unwrap())
        .stderr(OpenOptions::new().create(true).append(true).open(path.as_path()).unwrap());

    match daemonize.start() {
        Err(e) => eprintln!("Error, {}", e),
        _ => {}
    }
}