extern crate sort;
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use sort::*;

fn rust_unstable_5m(c: &mut Criterion) {
    let arr_size = 5_000_000;
    c.bench_function("rust unstable 5M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_10m(c: &mut Criterion) {
    let arr_size = 10_000_000;
    c.bench_function("rust unstable 10M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_15m(c: &mut Criterion) {
    let arr_size = 15_000_000;
    c.bench_function("rust unstable 15M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

fn rust_unstable_20m(c: &mut Criterion) {
    let arr_size = 20_000_000;
    c.bench_function("rust unstable 20M", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| vals.sort_unstable() ); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = rust_unstable_5m, rust_unstable_10m, rust_unstable_15m, rust_unstable_20m}
criterion_main!(benches);
