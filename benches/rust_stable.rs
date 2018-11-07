extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use rand::Rng;

fn rust_stable_10(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10).map(|_| {
        rng.gen_range(0, 10)
    }).collect();
    c.bench_function("rust stable 10", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

fn rust_stable_100(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100).map(|_| {
        rng.gen_range(0, 100)
    }).collect();
    c.bench_function("rust stable 100", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

fn rust_stable_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("rust stable 1k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

fn rust_stable_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("rust stable 10k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

fn rust_stable_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("rust stable 100k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

fn rust_stable_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("rust stable 1M", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| vals.sort()); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = rust_stable_10, rust_stable_100, rust_stable_1k, rust_stable_10k, rust_stable_100k, rust_stable_1m
}
criterion_main!(benches);
