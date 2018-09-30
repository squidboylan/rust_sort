extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn merge_sort_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("merge sort 1k", |b| b.iter(|| merge_sort(&mut numbers)));
}

fn merge_sort_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("merge sort 10k", |b| b.iter(|| merge_sort(&mut numbers)));
}

fn merge_sort_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("merge sort 100k", |b| b.iter(|| merge_sort(&mut numbers)));
}

fn merge_sort_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("merge sort 1M", |b| b.iter(|| merge_sort(&mut numbers)));
}

criterion_group!(benches, merge_sort_1k, merge_sort_10k, merge_sort_100k, merge_sort_1m);
criterion_main!(benches);
