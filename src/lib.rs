extern crate rand;
extern crate crossbeam;

use std::fmt::Debug;

pub fn insertion_sort<T: PartialOrd>(vals: &mut [T]) {
    let mut i = 1;
    while i < vals.len() {
        let mut j = i;
        while j > 0 && vals[j] < vals[j-1] {
            vals.swap(j, j-1);
            j -= 1;
        }
        i += 1;
    }
}


pub fn selection_sort<T: PartialOrd>(vals: &mut [T]) {
    if vals.len() == 0 {
        return;
    }
    let mut curr = vals.len() - 1;
    while curr > 0 {
        let mut i = curr;
        let mut j = curr;
        while j > 0 {
            if vals[j-1] > vals[i] {
                i = j-1;
            }
            j -= 1;
        }
        vals.swap(i, curr);
        curr -= 1;
    }
}


pub fn bubble_sort<T: PartialOrd>(vals: &mut [T]) {
    if vals.len() == 0 {
        return;
    }
    let mut end = vals.len() - 1;
    loop {
        let mut swapped = false;
        let mut i = 0;
        while i < end {
            if vals[i] > vals[i+1] {
                vals.swap(i, i+1);
                swapped = true;
            }
            i += 1;
        }

        if !swapped {
            break;
        }
        end -= 1;
    }
}

// basic Hoare partition without optimization
pub fn partition<T: PartialOrd + Copy>(vals: &mut [T]) -> usize {
    let pivot = vals[0];
    let mut i = 1;
    let mut j = vals.len() - 1;
    loop {
        while i < vals.len() && vals[i] < pivot {
            i += 1;
        }
        while vals[j] >= pivot {
            if j == 0 {
                break;
            }
            j -= 1;
        }

        if i >= j {
            break;
        }
        vals.swap(i, j);
        i += 1;
        j -= 1;
    }
    vals.swap(0, j);
    j
}

// Optimized Hoare partition that takes the median of 3 values to find a
// non-worst case pivot
pub fn partition_opt<T: PartialOrd + Copy + Debug>(vals: &mut [T]) -> usize {
    let mut pivot: T = vals[0];
    if vals.len() > 2 {
        let v1 = vals[0];
        let v2 = vals[vals.len()/2];
        let v3 = vals[vals.len() - 1];
        //println!("v1 {:?}, v2: {:?}, v3: {:?}", v1, v2, v3);
        if (v1 <= v2 && v2 <= v3) || (v3 <= v2 && v2 <= v1) {
            pivot = v2;
            let pivot_index = vals.len()/2;
            vals.swap(0, pivot_index);
        } else if (v1 <= v3 && v3 <= v2) || (v2 <= v3 && v3 <= v1) {
            pivot = v3;
            let pivot_index = vals.len() - 1;
            vals.swap(0, pivot_index);
        } else {
            pivot = v1;
        }
    }
    let mut i = 1;
    let mut j = vals.len() - 1;
    loop {
        while i < vals.len() && vals[i] < pivot {
            i += 1;
        }
        while vals[j] >= pivot {
            if j == 0 {
                break;
            }
            j -= 1;
        }

        if i >= j {
            break;
        }
        vals.swap(i, j);
        i += 1;
        j -= 1;
    }
    vals.swap(0, j);
    //println!("len: {:?}, Pivot: {:?}, index: {:?}", vals.len(), pivot, j);

    j
}


// Quicksort that uses basic Hoare partitioning
pub fn quicksort<T: PartialOrd + Copy + Debug>(vals: &mut [T]) {
    if vals.len() > 1 {
        let pivot = partition(vals);
        let tmp = vals.split_at_mut(pivot);
        quicksort(tmp.0);
        quicksort(tmp.1.split_at_mut(1).1);
    }
}


// Quicksort that uses optimized Hoare partitioning
pub fn quicksort_opt<T: PartialOrd + Copy + Debug>(vals: &mut [T]) {
    if vals.len() <= 1 {
        return;
    }
    let pivot = partition_opt(vals);
    let tmp = vals.split_at_mut(pivot);
    quicksort_opt(tmp.0);
    quicksort_opt(tmp.1.split_at_mut(1).1);
}

pub fn merge_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    if vals.len() > 1 {
        let mut chunks = Vec::new();
        {
            let tmp = vals.split_at(vals.len()/2);
            chunks.push(tmp.0.to_vec());
            chunks.push(tmp.1.to_vec());
        }
        merge_sort(&mut chunks[0]);
        merge_sort(&mut chunks[1]);

        merge(vals, &mut chunks);
    }
}

pub fn optimized_merge_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    if vals.len() > 1 {
        if vals.len() <= 20 {
            insertion_sort(vals);
            return;
        }
        let mut chunks = Vec::new();
        {
            let tmp = vals.split_at(vals.len()/2);
            chunks.push(tmp.0.to_vec());
            chunks.push(tmp.1.to_vec());
        }
        merge_sort(&mut chunks[0]);
        merge_sort(&mut chunks[1]);

        merge(vals, &mut chunks);
    }
}

pub fn merge<T: PartialOrd + Copy>(vals: &mut [T], chunks: &mut [Vec<T>]) {
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    while x < chunks[0].len() && y < chunks[1].len() {
        if chunks[0][x] <= chunks[1][y] {
            vals[i] = chunks[0][x];
            x += 1;
            i += 1;
        } else if chunks[0][x] > chunks[1][y] {
            vals[i] = chunks[1][y];
            y += 1;
            i += 1;
        }
    }
    while x < chunks[0].len() {
        vals[i] = chunks[0][x];
        x += 1;
        i += 1;
    }
    while y < chunks[1].len() {
        vals[i] = chunks[1][y];
        y += 1;
        i += 1;
    }

}


pub fn merge_sort_multithreaded<T: PartialOrd + Copy + Send>(vals: &mut [T], depth: usize) {
    if vals.len() == 1 {
        return;
    }
    let mut chunks = Vec::new();
    {
        let tmp = vals.split_at(vals.len()/2);
        chunks.push(tmp.0.to_vec());
        chunks.push(tmp.1.to_vec());
    }
    if depth > 0 {
        crossbeam::scope(|scope| {
            for i in &mut chunks {
                scope.spawn(move || {
                    merge_sort_multithreaded(i, depth - 1);
                });
            }
        });
    } else {
        merge_sort(&mut chunks[0]);
        merge_sort(&mut chunks[1]);
    }
    merge(vals, &mut chunks);
}


pub fn optimized_merge_sort_multithreaded<T: PartialOrd + Copy + Send>(vals: &mut [T], depth: usize) {
    if vals.len() == 1 {
        return;
    }
    if vals.len() <= 20 {
        insertion_sort(vals);
        return;
    }
    let mut chunks = Vec::new();
    {
        let tmp = vals.split_at(vals.len()/2);
        chunks.push(tmp.0.to_vec());
        chunks.push(tmp.1.to_vec());
    }
    if depth > 0 {
        crossbeam::scope(|scope| {
            for i in &mut chunks {
                scope.spawn(move || {
                    optimized_merge_sort_multithreaded(i, depth - 1);
                });
            }
        });
    } else {
        optimized_merge_sort(&mut chunks[0]);
        optimized_merge_sort(&mut chunks[1]);
    }
    merge(vals, &mut chunks);
}


#[cfg(test)]
mod tests {

    use rand::Rng;
    use super::*;

    fn test_function(f: fn(&mut [u64])) {
        let mut rng = rand::thread_rng();
        for i in 0..1024 {
            let mut numbers: Vec<u64> = (0..i).map(|_| {
                rng.gen_range(0, i)
            }).collect();
            let mut sorted = numbers.clone();
            sorted.sort_unstable();
            f(numbers.as_mut_slice());
            assert_eq!(numbers, sorted);
        }
    }

    fn test_function_mt(f: fn(&mut [u64], usize)) {
        let mut rng = rand::thread_rng();
        for i in 0..1024 {
            for depth in 0..3 {
                let mut numbers: Vec<u64> = (0..i).map(|_| {
                    rng.gen_range(0, i)
                }).collect();
                let mut sorted = numbers.clone();
                sorted.sort_unstable();

                f(numbers.as_mut_slice(), depth);
                assert_eq!(numbers, sorted);
            }
        }
    }

    #[test]
    fn insertion_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        insertion_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function(insertion_sort);
    }

    #[test]
    fn bubble_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        bubble_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function(bubble_sort);
    }

    #[test]
    fn merge_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function(merge_sort);
    }

    #[test]
    fn merge_sort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function_mt(merge_sort_multithreaded);
    }

    #[test]
    fn optimized_merge_sort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        optimized_merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        optimized_merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function_mt(optimized_merge_sort_multithreaded);
    }

    #[test]
    fn selection_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        selection_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);
        test_function(selection_sort);
    }

    #[test]
    fn quicksort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8, 9, 10];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut vals = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        let mut sorted = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        sorted.sort();
        quicksort(&mut vals);
        assert_eq!(vals, sorted);
        test_function(quicksort);
    }

    #[test]
    fn quicksort_opt_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        quicksort_opt(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        quicksort_opt(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8, 9, 10];
        quicksort_opt(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut vals = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        let mut sorted = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        sorted.sort();
        quicksort_opt(&mut vals);
        assert_eq!(vals, sorted);
        test_function(quicksort_opt);
    }
}
