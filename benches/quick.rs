extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn quicksort_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("quicksort 1T 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("quicksort 1T 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_1k(c: &mut Criterion) {
    let arr_size = 1_000;
    c.bench_function("quicksort 1T 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("quicksort 1T 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("quicksort 1T 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_1m(c: &mut Criterion) {
    let arr_size = 1_000_000;
    c.bench_function("quicksort 1T 1M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = quicksort_10, quicksort_100, quicksort_1k, quicksort_10k, quicksort_100k, quicksort_1m}
criterion_main!(benches);
