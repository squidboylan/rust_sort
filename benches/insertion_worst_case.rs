extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn insertion_sort_worst_case_1k(c: &mut Criterion) {
    c.sample_size(10);

    let mut numbers: Vec<u64> = (0..1_000).map(|i| {
        1_000 - i
    }).collect();
    c.bench_function("insertion sort worst case 1k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

fn insertion_sort_worst_case_10k(c: &mut Criterion) {
    c.sample_size(10);

    let mut numbers: Vec<u64> = (0..10_000).map(|i| {
        10_000 - i
    }).collect();
    c.bench_function("insertion sort worst case 10k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

fn insertion_sort_worst_case_100k(c: &mut Criterion) {
    c.sample_size(10);

    let mut numbers: Vec<u64> = (0..100_000).map(|i| {
        100_000 - i
    }).collect();
    c.bench_function("insertion sort worst case 100k", |b| b.iter(|| insertion_sort(&mut numbers.clone())));
}

criterion_group!(benches, insertion_sort_worst_case_1k, insertion_sort_worst_case_10k, insertion_sort_worst_case_100k);
criterion_main!(benches);
