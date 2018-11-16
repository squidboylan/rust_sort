extern crate rand;
extern crate crossbeam;
extern crate rayon;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

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
    let mut i = 0;
    let mut j = vals.len() - 1;
    loop {
        while vals[i] < pivot {
            i += 1;
        }
        while vals[j] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }
        vals.swap(i, j);
    }
}

pub fn quicksort<T: PartialOrd + Copy>(vals: &mut [T]) {
    if vals.len() > 1 {
        let pivot = partition(vals);
        let tmp = vals.split_at_mut(pivot + 1);
        quicksort(tmp.0.split_last_mut().unwrap().1);
        quicksort(tmp.1);
    }
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

        chunks.par_iter_mut().for_each(|e| {
            merge_sort_multithreaded(e, depth - 1)
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

        chunks.par_iter_mut().for_each(|e| {
            optimized_merge_sort_multithreaded(e, depth - 1)
        });
    } else {
        optimized_merge_sort(&mut chunks[0]);
        optimized_merge_sort(&mut chunks[1]);
    }
    merge(vals, &mut chunks);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        insertion_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn bubble_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        bubble_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn merge_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn merge_sort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn optimized_merge_sort_multithreaded_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        optimized_merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        optimized_merge_sort_multithreaded(&mut vals, 3);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn selection_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        selection_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn quicksort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        quicksort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }
}
