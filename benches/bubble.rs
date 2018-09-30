extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn bubble_sort_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("bubble sort 1k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn bubble_sort_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("bubble sort 10k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn bubble_sort_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("bubble sort 100k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn bubble_sort_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("bubble sort 1M", |b| b.iter(|| insertion_sort(&mut numbers)));
}

criterion_group!(benches, bubble_sort_1k, bubble_sort_10k, bubble_sort_100k, bubble_sort_1m);
criterion_main!(benches);
