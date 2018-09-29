extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn insertion_sort_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("insertion sort 1k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn insertion_sort_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("insertion sort 10k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn insertion_sort_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("insertion sort 100k", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn insertion_sort_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("insertion sort 1M", |b| b.iter(|| insertion_sort(&mut numbers)));
}

fn insertion_sort_10m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10_000_000).map(|_| {
        rng.gen_range(0, 10_000_000)
    }).collect();
    c.bench_function("insertion sort 10M", |b| b.iter(|| insertion_sort(&mut numbers)));
}

criterion_group!(benches, insertion_sort_1k, insertion_sort_10k, insertion_sort_100k, insertion_sort_1m, insertion_sort_10m);
criterion_main!(benches);
