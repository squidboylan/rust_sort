extern crate sort;
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sort::*;

fn rust_unstable_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("rust unstable 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("rust unstable 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_1k(c: &mut Criterion) {
    let arr_size = 1_000;
    c.bench_function("rust unstable 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("rust unstable 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("rust unstable 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_1m(c: &mut Criterion) {
    let arr_size = 1_000_000;
    c.bench_function("rust unstable 1M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = rust_unstable_10, rust_unstable_100, rust_unstable_1k, rust_unstable_10k, rust_unstable_100k, rust_unstable_1m
}
criterion_main!(benches);
