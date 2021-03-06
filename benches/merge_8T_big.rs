extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn merge_sort_5m(c: &mut Criterion) {
    let arr_size = 5_000_000;
    c.bench_function("merge sort 8T 5M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 3)); } );
}

fn merge_sort_10m(c: &mut Criterion) {
    let arr_size = 10_000_000;
    c.bench_function("merge sort 8T 10M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 3)); } );
}

fn merge_sort_15m(c: &mut Criterion) {
    let arr_size = 15_000_000;
    c.bench_function("merge sort 8T 15M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 3)); } );
}

fn merge_sort_20m(c: &mut Criterion) {
    let arr_size = 20_000_000;
    c.bench_function("merge sort 8T 20M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 3)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = merge_sort_5m, merge_sort_10m, merge_sort_15m, merge_sort_20m}
criterion_main!(benches);
