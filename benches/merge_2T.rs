extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn merge_sort_threaded_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("merge sort 2T 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

fn merge_sort_threaded_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("merge sort 2T 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

fn merge_sort_threaded_1k(c: &mut Criterion) {
    let arr_size = 1000;
    c.bench_function("merge sort 2T 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

fn merge_sort_threaded_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("merge sort 2T 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

fn merge_sort_threaded_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("merge sort 2T 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

fn merge_sort_threaded_1m(c: &mut Criterion) {
    let arr_size = 1_000_000;
    c.bench_function("merge sort 2T 1M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| merge_sort_multithreaded(&mut vals, 1)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = merge_sort_threaded_10, merge_sort_threaded_100, merge_sort_threaded_1k, merge_sort_threaded_10k, merge_sort_threaded_100k, merge_sort_threaded_1m
}
criterion_main!(benches);
