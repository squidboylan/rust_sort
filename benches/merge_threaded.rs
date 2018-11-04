extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn merge_sort_threaded_10(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10).map(|_| {
        rng.gen_range(0, 10)
    }).collect();
    c.bench_function("merge sort multithreaded 10", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

fn merge_sort_threaded_100(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100).map(|_| {
        rng.gen_range(0, 100)
    }).collect();
    c.bench_function("merge sort multithreaded 100", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

fn merge_sort_threaded_1k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("merge sort multithreaded 1k", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

fn merge_sort_threaded_10k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("merge sort multithreaded 10k", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

fn merge_sort_threaded_100k(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("merge sort multithreaded 100k", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

fn merge_sort_threaded_1m(c: &mut Criterion) {
    c.sample_size(10);
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("merge sort multithreaded 1M", |b| b.iter(|| merge_sort_multithreaded(&mut numbers.clone(), 3)));
}

criterion_group!(benches, merge_sort_threaded_10, merge_sort_threaded_100, merge_sort_threaded_1k, merge_sort_threaded_10k, merge_sort_threaded_100k, merge_sort_threaded_1m);
criterion_main!(benches);
