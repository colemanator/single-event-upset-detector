# Single Event Upset Detector
Small and very simple program to detect single events upset. Inspired by a [video] from Veritasium,
specifically a quoted stat that for every 250Mb of memory one upset event should occur per month.
So in theory if you left this program running with 250Mb of memory over an entire month 
statistically you should see one single event upset. I also took the opportunity to experiment with
SIMD intrinsics and learnt a lot there.

If running this program over a long period of time you might want to use `launchd` on OSX or 
`systemd` on Linux to ensure it stays running even after a reboot.  

_This program will not work if run on hardware which has error correction code memory._

## Requirements
In order to build/run this program you'll need to be using the latest nightly release:
```asm
rustup update -- nightly
```

You'll want to use nightly when building/running, which you can do by using:
```asm
rustup default nightly
```

_Alternatively you can prefix cargo sub commands with `+nightly`._

## Installing
You can install this program run:
```asm
cargo install --path .
```

## Usage
The usage for this program is as follows:

```asm
single-event-upset-detector [FLAGS] <bytes> <interval>
```

__Arguments:__
* `bytes` The number of bytes to use when scanning for a single event upset 
* `interval` The number of seconds to wait in between scans

__Options:__
* `-f --file` Output to file when process is a daemon
* `-d --daemon` Run as daemon
* `-v --verbose` log diagnostics

_Use the `--help` flag for more info on usage._

The program will fill the directed number of bytes using 64-bit unsigned integers all with a value 
of `0`, the integers are aligned for use in an 8 lane vector. It will than scan the memory, waiting 
the provided interval in between scans. If a single event upset is detected details will be written
to `stdout` in the following format:

```asm
Single Event Upset detected - timestamp: {timestamp}, address: {address}, bits: {bits}, value: {value}
```

* `timestamp` the time at which the single event upset was detected
* `address` the address of the integer in hex
* `bits` the bits comprising the integer seperated by `|`
* `value` the new value of the integer (it was `0`)

_You don't need a low interval but it might help keep the data uncompressed or help avoid swap (if
either of those are a concern).

## Performance
Using SIMD directly instead of relying on LLVM's auto-vectorization I was able to improve 
performance slightly, you can see a comparison by running the benchmarks using `cargo bench`.
Running on a Intel core i7-7920HQ with 2133 MHz LPDDR3 memory it was able to scan over 12 GBs.
What's probably more important is efficiency, this program will obviously use whatever memory you
tell it to, beyond that it's single threaded and shouldn't have a noticeable impact on system 
responsiveness.

[video]: https://www.youtube.com/watch?v=AaZ_RSt0KP8