extern crate rand;
extern crate crossbeam;

use rand::Rng;

#[inline(always)]
pub fn rand_vec_u64(size: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| { rng.gen_range(0, size) }).collect()
}

#[inline(always)]
pub fn reverse_sorted_vec_u64(size: u64) -> Vec<u64> {
    (0..size).map(|i| { size - i }).collect()
}

#[inline(always)]
pub fn sorted_vec_u64(size: u64) -> Vec<u64> {
    (0..size).map(|i| { i }).collect()
}

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
pub fn partition<T: PartialOrd>(vals: &mut [T]) -> usize {
    let mut i = 1;
    let mut j = vals.len() - 1;
    loop {
        while i < vals.len() && vals[i] < vals[0] {
            i += 1;
        }
        while vals[j] >= vals[0] {
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

// Quicksort that uses basic Hoare partitioning
pub fn quicksort<T: PartialOrd>(vals: &mut [T]) {
    if vals.len() > 1 {
        let pivot = partition(vals);
        // Split vals into two arrays, left contains [0, pivot), the contains (pivot, len)
        let tmp = vals.split_at_mut(pivot);
        let left = tmp.0;
        let right = tmp.1.split_at_mut(1).1;
        quicksort(left);
        quicksort(right);
    }
}

// Multithreaded quicksort that uses basic Hoare partitioning
pub fn quicksort_multithreaded<T: PartialOrd + Send>(vals: &mut [T], depth: usize) {
    if vals.len() > 1 {
        let pivot = partition(vals);
        // Split vals into two arrays, left contains [0, pivot), the contains (pivot, len)
        let tmp = vals.split_at_mut(pivot);
        let left = tmp.0;
        let right = tmp.1.split_at_mut(1).1;
        if depth > 1 {
            // We need crossbeam scoped threads here to appease the lifetime checker gods
            // This spawns 1 thread to sort each half of the array and waits for them to finish
            crossbeam::scope(|scope| {
                scope.spawn(|| {
                    quicksort_multithreaded(left, depth - 1);
                });
                scope.spawn(|| {
                    quicksort_multithreaded(right, depth - 1);
                });
            });
        } else {
            quicksort(left);
            quicksort(right);
        }
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    if vals.len() > 1 {
        let mut left;
        let mut right;
        {
            let tmp = vals.split_at(vals.len()/2);
            left = tmp.0.to_vec();
            right = tmp.1.to_vec();
        }
        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(vals, &mut left, &mut right);
    }
}

pub fn merge<T: PartialOrd + Copy>(vals: &mut [T], left: &mut [T], right: &mut [T]) {
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    while x < left.len() && y < right.len() {
        if left[x] <= right[y] {
            vals[i] = left[x];
            x += 1;
            i += 1;
        } else if left[x] > right[y] {
            vals[i] = right[y];
            y += 1;
            i += 1;
        }
    }
    while x < left.len() {
        vals[i] = left[x];
        x += 1;
        i += 1;
    }
    while y < right.len() {
        vals[i] = right[y];
        y += 1;
        i += 1;
    }

}


pub fn merge_sort_multithreaded<T: PartialOrd + Copy + Send>(vals: &mut [T], depth: usize) {
    if vals.len() == 1 {
        return;
    }
    // copy each half of our vector into new vectors to be merge sorted
    let mut left;
    let mut right;
    {
        let tmp = vals.split_at(vals.len()/2);
        left = tmp.0.to_vec();
        right = tmp.1.to_vec();
    }
    // If depth > 0 we need to spawn new threads in this call
    if depth > 0 {
        // We need crossbeam scoped threads here to appease the lifetime checker gods
        // This spawns 1 thread to sort each half of the array and waits for them to finish
        crossbeam::scope(|scope| {
            scope.spawn(|| {
                merge_sort_multithreaded(&mut left, depth - 1);
            });
            scope.spawn(|| {
                merge_sort_multithreaded(&mut right, depth - 1);
            });
        });
    } else {
        merge_sort(&mut left);
        merge_sort(&mut right);
    }
    merge(vals, &mut left, &mut right);
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test a sorting function that takes an array as an argument
    fn test_function_rand(f: fn(&mut [u64])) {
        for i in 0..1024 {
            let mut numbers = rand_vec_u64(i);
            let mut sorted = numbers.clone();
            sorted.sort_unstable();
            f(numbers.as_mut_slice());
            assert_eq!(numbers, sorted);
        }
    }

    fn test_function_sorted(f: fn(&mut [u64])) {
        for i in 0..1024 {
            let mut numbers = sorted_vec_u64(i);
            let mut sorted = numbers.clone();
            sorted.sort_unstable();
            f(numbers.as_mut_slice());
            assert_eq!(numbers, sorted);
        }
    }

    fn test_function_reverse_sorted(f: fn(&mut [u64])) {
        for i in 0..1024 {
            let mut numbers = reverse_sorted_vec_u64(i);
            let mut sorted = numbers.clone();
            sorted.sort_unstable();
            f(numbers.as_mut_slice());
            assert_eq!(numbers, sorted);
        }
    }

    // Test a sorting function that takes an array and depth of multithreading
    // as arguments
    fn test_function_rand_mt(f: fn(&mut [u64], usize)) {
        for i in 0..1024 {
            for depth in 0..3 {
                let mut numbers = rand_vec_u64(i);
                let mut sorted = numbers.clone();
                sorted.sort_unstable();

                f(numbers.as_mut_slice(), depth);
                assert_eq!(numbers, sorted);
            }
        }
    }

    fn test_function_sorted_mt(f: fn(&mut [u64], usize)) {
        for i in 0..1024 {
            for depth in 0..3 {
                let mut numbers = sorted_vec_u64(i);
                let mut sorted = numbers.clone();
                sorted.sort_unstable();

                f(numbers.as_mut_slice(), depth);
                assert_eq!(numbers, sorted);
            }
        }
    }

    fn test_function_reverse_sorted_mt(f: fn(&mut [u64], usize)) {
        for i in 0..1024 {
            for depth in 0..3 {
                let mut numbers = reverse_sorted_vec_u64(i);
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
        test_function_rand(insertion_sort);
        test_function_sorted(insertion_sort);
        test_function_reverse_sorted(insertion_sort);
    }

    #[test]
    fn bubble_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        bubble_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function_rand(bubble_sort);
        test_function_sorted(bubble_sort);
        test_function_reverse_sorted(bubble_sort);
    }

    #[test]
    fn merge_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function_rand(merge_sort);
        test_function_sorted(merge_sort);
        test_function_reverse_sorted(merge_sort);
    }

    #[test]
    fn merge_sort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
        test_function_rand_mt(merge_sort_multithreaded);
        test_function_sorted_mt(merge_sort_multithreaded);
        test_function_reverse_sorted_mt(merge_sort_multithreaded);
    }

    #[test]
    fn selection_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        selection_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);
        test_function_rand(selection_sort);
        test_function_sorted(selection_sort);
        test_function_reverse_sorted(selection_sort);
    }

    #[test]
    fn quicksort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

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
        test_function_rand(quicksort);
        test_function_sorted(quicksort);
        test_function_reverse_sorted(quicksort);
    }

    #[test]
    fn quicksort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        quicksort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        quicksort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8, 9, 10];
        quicksort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut vals = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        let mut sorted = [7, 8, 9, 1, 1, 5, 9, 2, 6, 5];
        sorted.sort();
        quicksort_multithreaded(&mut vals, 3);
        assert_eq!(vals, sorted);
        test_function_rand_mt(quicksort_multithreaded);
        test_function_sorted_mt(quicksort_multithreaded);
        test_function_reverse_sorted_mt(quicksort_multithreaded);
    }
}
