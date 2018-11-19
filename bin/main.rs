extern crate sort;
use sort::*;

pub fn main() {
    let mut nums: Vec<u64> = (0..1_000_000).rev().collect();
    //let mut nums: Vec<u64> = (0..1_000_000).collect();
    quicksort_opt(&mut nums);
}
