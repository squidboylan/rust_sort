extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use rand::Rng;

fn rust_unstable_10(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10).map(|_| {
        rng.gen_range(0, 10)
    }).collect();
    c.bench_function("rust unstable 10", |b| b.iter(|| numbers.clone().sort()));
}

fn rust_unstable_100(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100).map(|_| {
        rng.gen_range(0, 100)
    }).collect();
    c.bench_function("rust unstable 100", |b| b.iter(|| numbers.clone().sort()));
}

fn rust_unstable_1k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("rust unstable 1k", |b| b.iter(|| numbers.clone().sort()));
}

fn rust_unstable_10k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("rust unstable 10k", |b| b.iter(|| numbers.clone().sort()));
}

fn rust_unstable_100k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("rust unstable 100k", |b| b.iter(|| numbers.clone().sort()));
}

fn rust_unstable_1m(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("rust unstable 1M", |b| b.iter(|| numbers.clone().sort()));
}

criterion_group!(benches, rust_unstable_10, rust_unstable_100, rust_unstable_1k, rust_unstable_10k, rust_unstable_100k, rust_unstable_1m);
criterion_main!(benches);
