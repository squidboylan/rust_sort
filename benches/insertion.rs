extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn insertion_sort_1k(c: &mut Criterion) {
    c.sample_size(10);
    let c = c.with_plots();
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("insertion sort 1k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

fn insertion_sort_10k(c: &mut Criterion) {
    c.sample_size(10);
    let c = c.with_plots();
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("insertion sort 10k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

fn insertion_sort_100k(c: &mut Criterion) {
    c.sample_size(10);
    let c = c.with_plots();
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("insertion sort 100k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

criterion_group!(benches, insertion_sort_1k, insertion_sort_10k, insertion_sort_100k);
criterion_main!(benches);
