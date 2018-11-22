extern crate sort;
#[macro_use]
extern crate criterion;

use sort::*;
use criterion::Criterion;

// Generate a reverse sorted vec of items 0 to size-1
fn generate_vec(size: u64) -> Vec<u64> {
    (0..size).rev().collect()
}

fn quicksort_worst_case_10(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(10);
    c.bench_function("quicksort worst case 10", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_worst_case_100(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(100);
    c.bench_function("quicksort worst case 100", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_worst_case_1k(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(1_000);
    c.bench_function("quicksort worst case 1K", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_worst_case_10k(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(10_000);
    c.bench_function("quicksort worst case 10K", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_worst_case_100k(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(100_000);
    c.bench_function("quicksort worst case 100K", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

fn quicksort_worst_case_1m(c: &mut Criterion) {
    let numbers: Vec<u64> = generate_vec(1_000_000);
    c.bench_function("quicksort worst case 1M", move |b| { b.iter_with_setup(|| numbers.clone(), move |mut vals: Vec<u64>| quicksort(&mut vals)); } );
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = quicksort_worst_case_10, quicksort_worst_case_100, quicksort_worst_case_1k, quicksort_worst_case_10k, quicksort_worst_case_100k, quicksort_worst_case_1m}
criterion_main!(benches);
