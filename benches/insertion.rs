extern crate sort;
#[macro_use]
extern crate criterion;
extern crate rand;

use sort::*;
use criterion::Criterion;
use rand::Rng;

fn insertion_sort_1k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..1_000).map(|_| {
        rng.gen_range(0, 1_000)
    }).collect();
    c.bench_function("insertion sort 1k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_10k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..10_000).map(|_| {
        rng.gen_range(0, 10_000)
    }).collect();
    c.bench_function("insertion sort 10k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

fn insertion_sort_100k(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u64> = (0..100_000).map(|_| {
        rng.gen_range(0, 100_000)
    }).collect();
    c.bench_function("insertion sort 100k", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| insertion_sort(&mut vals)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = insertion_sort_1k, insertion_sort_10k, insertion_sort_100k
}
criterion_main!(benches);
