use criterion::{black_box, criterion_group, criterion_main, Criterion};
use single_event_upset_detector::detector::{Detector, SingleEventUpset};

fn vector_man(c: &mut Criterion) {
    let detector = Detector::new(8_000_000);

    c.bench_function("Vectorisation - Man", |b| {
        b.iter(|| {
            let upsets = detector.get_upsets();

            // Force the compiler to use the values
            for upset in upsets {
                black_box(upset);
            }
        })
    });
}

fn vector_auto(c: &mut Criterion) {
    c.bench_function("Vectorisation - Auto", |b| {
        let nums: Vec<u64> = vec![0; 1_000_000];

        b.iter(|| {
            let upsets = black_box(&nums)
                .iter()
                .filter(|num| **num != 0)
                .map(|num| SingleEventUpset::new(num));

            // Force the compiler to use the values
            for upset in upsets {
                black_box(upset);
            }
        })
    });
}

criterion_group!(benches, vector_man, vector_auto);
criterion_main!(benches);