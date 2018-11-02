extern crate rand;
extern crate crossbeam;
extern crate rayon;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

const NUM_THREADS: usize = 8;

pub fn insertion_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
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


pub fn selection_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
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


pub fn bubble_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
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

pub fn merge_sort<T: PartialOrd + Copy + Clone>(vals: &mut [T]) {
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

pub fn merge<T: PartialOrd + Copy + Clone>(vals: &mut [T], chunks: &mut [Vec<T>]) {
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


pub fn merge_sort_multithreaded<T: PartialOrd + Copy + Clone + Send>(vals: &mut [T]) {
    if vals.len() >= 2 {
        let mut chunks: Vec<Vec<T>> = Vec::new();
        {
            let tmp = vals.chunks(vals.len()/NUM_THREADS + 1);
            for i in tmp {
                chunks.push(i.to_vec());
            }
        }

        chunks.par_iter_mut().for_each(|e| {
            merge_sort(e)
        });

        merge(vals, &mut chunks);
    }

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
    fn selection_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3, 8];
        selection_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
