extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn quicksort_threaded_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("quicksort 4T 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

fn quicksort_threaded_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("quicksort 4T 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

fn quicksort_threaded_1k(c: &mut Criterion) {
    let arr_size = 1000;
    c.bench_function("quicksort 4T 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

fn quicksort_threaded_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("quicksort 4T 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

fn quicksort_threaded_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("quicksort 4T 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

fn quicksort_threaded_1m(c: &mut Criterion) {
    let arr_size = 1_000_000;
    c.bench_function("quicksort 4T 1M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort_multithreaded(&mut vals, 2)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = quicksort_threaded_10, quicksort_threaded_100, quicksort_threaded_1k, quicksort_threaded_10k, quicksort_threaded_100k, quicksort_threaded_1m
}
criterion_main!(benches);
