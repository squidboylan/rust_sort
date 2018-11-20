extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn merge_sort_threaded_10(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10).map(|_| {
        rng.gen_range(0, 10)
    }).collect();
    c.bench_function("merge sort 4T 10", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

fn merge_sort_threaded_100(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100).map(|_| {
        rng.gen_range(0, 100)
    }).collect();
    c.bench_function("merge sort 4T 100", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

fn merge_sort_threaded_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("merge sort 4T 1k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

fn merge_sort_threaded_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("merge sort 4T 10k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

fn merge_sort_threaded_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("merge sort 4T 100k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

fn merge_sort_threaded_1m(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000_000).map(|_| {
        rng.gen_range(0, 1_000_000)
    }).collect();
    c.bench_function("merge sort 4T 1M", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 2)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = merge_sort_threaded_10, merge_sort_threaded_100, merge_sort_threaded_1k, merge_sort_threaded_10k, merge_sort_threaded_100k, merge_sort_threaded_1m
}
criterion_main!(benches);
