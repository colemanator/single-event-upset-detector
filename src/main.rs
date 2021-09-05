#![feature(test)]
extern crate test;

use test::{Bencher, black_box};
use packed_simd::{u8x4, u8x64, u64x8};

fn main() {}

#[bench]
fn linear_detect(b: &mut Bencher) {
    let set: Vec<u64> = black_box(vec![0; 32_000_000]);

    b.iter(|| {
        set.iter().fold(0, |acc, &x| acc + x) > 0
    });
}

#[bench]
fn linear_identify(b: &mut Bencher) {
    let mut set: Vec<u64> = black_box(vec![0; 32_000_000]);

    b.iter(|| -> Vec<usize>{
        set.iter().enumerate().filter(|(i, n)| **n != 0).map(|(i, n)| i).collect()
    });
}

#[bench]
fn vector_detect(b: &mut Bencher) {
    let set: Vec<u64> = black_box(vec![0; 32_000_000]);

    b.iter(|| {
        let mut sum = u64x8::splat(0);

        for i in (0..set.len()).step_by(64) {
            // We control allocation so the slice is always guaranteed to be aligned
            unsafe { sum += u64x8::from_slice_aligned_unchecked(&set[i..]); }
        }

        sum.wrapping_sum() > 0
    });
}

#[bench]
fn vector_identify(b: &mut Bencher) {
    let set: Vec<u64> = black_box(vec![0; 32_000_000]);

    b.iter(|| {
        let mut expected = u64x8::splat(0);
        let mut upset: Vec<usize> = Vec::new();

        for i in (0..set.len()).step_by(64) {
            unsafe {
                // We control allocation so the slice is always guaranteed to be aligned
                if expected != u64x8::from_slice_aligned_unchecked(&set[i..]) {
                    // Record the index(s) at which the upset occurred
                    upset.extend(
                        set[i..]
                            .iter()
                            .take(64)
                            .enumerate()
                            .filter(|(i, n)| **n != 0)
                            .map(|(i, n)| i)
                            .into_iter()
                    );
                }
            }
        }

        upset
    });
}

