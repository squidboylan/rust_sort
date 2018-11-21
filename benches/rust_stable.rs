extern crate sort;
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sort::*;

fn rust_stable_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("rust stable 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

fn rust_stable_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("rust stable 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

fn rust_stable_1k(c: &mut Criterion) {
    let arr_size = 1_000;
    c.bench_function("rust stable 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

fn rust_stable_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("rust stable 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

fn rust_stable_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("rust stable 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

fn rust_stable_1m(c: &mut Criterion) {
    let arr_size = 1_000_000;
    c.bench_function("rust stable 1M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort() ); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = rust_stable_10, rust_stable_100, rust_stable_1k, rust_stable_10k, rust_stable_100k, rust_stable_1m
}
criterion_main!(benches);
