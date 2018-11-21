extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn bubble_sort_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("bubble sort 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| bubble_sort(&mut vals)); } );
}

fn bubble_sort_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("bubble sort 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| bubble_sort(&mut vals)); } );
}

fn bubble_sort_1k(c: &mut Criterion) {
    let arr_size = 1_000;
    c.bench_function("bubble sort 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| bubble_sort(&mut vals)); } );
}

fn bubble_sort_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("bubble sort 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| bubble_sort(&mut vals)); } );
}

fn bubble_sort_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("bubble sort 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| bubble_sort(&mut vals)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bubble_sort_10, bubble_sort_100, bubble_sort_1k, bubble_sort_10k, bubble_sort_100k
}
criterion_main!(benches);
