extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

fn insertion_sort_10(c: &mut Criterion) {
    let arr_size = 10;
    c.bench_function("insertion sort 10", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_100(c: &mut Criterion) {
    let arr_size = 100;
    c.bench_function("insertion sort 100", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_1k(c: &mut Criterion) {
    let arr_size = 1_000;
    c.bench_function("insertion sort 1k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_10k(c: &mut Criterion) {
    let arr_size = 10_000;
    c.bench_function("insertion sort 10k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_100k(c: &mut Criterion) {
    let arr_size = 100_000;
    c.bench_function("insertion sort 100k", move |b| { b.iter_with_setup(|| rand_vec_u64(arr_size), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = insertion_sort_10, insertion_sort_100, insertion_sort_1k, insertion_sort_10k, insertion_sort_100k
}
criterion_main!(benches);
