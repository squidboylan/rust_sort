extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use rand::Rng;

fn rust_unstable_10(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10).map(|_| {
        rng.gen_range(0, 10)
    }).collect();
    c.bench_function("rust unstable 10", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

fn rust_unstable_100(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100).map(|_| {
        rng.gen_range(0, 100)
    }).collect();
    c.bench_function("rust unstable 100", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

fn rust_unstable_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("rust unstable 1k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

fn rust_unstable_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("rust unstable 10k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

fn rust_unstable_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("rust unstable 100k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

fn rust_unstable_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("rust unstable 1M", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort_unstable()); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = rust_unstable_10, rust_unstable_100, rust_unstable_1k, rust_unstable_10k, rust_unstable_100k, rust_unstable_1m
}
criterion_main!(benches);
