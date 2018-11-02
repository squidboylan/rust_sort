extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn selection_sort_1k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("selection sort 1k", |b| b.iter(|| selection_sort(&mut numbers.clone())));
}

fn selection_sort_10k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("selection sort 10k", |b| b.iter(|| selection_sort(&mut numbers.clone())));
}

fn selection_sort_100k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("selection sort 100k", |b| b.iter(|| selection_sort(&mut numbers.clone())));
}

fn selection_sort_1m(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("selection sort 1M", |b| b.iter(|| selection_sort(&mut numbers.clone())));
}

criterion_group!(benches, selection_sort_1k, selection_sort_10k, selection_sort_100k);
criterion_main!(benches);
