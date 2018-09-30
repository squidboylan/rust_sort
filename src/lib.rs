extern crate rand;

pub fn insertion_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    let mut i = 1;
    while i < vals.len() {
        let mut j = i;
        while j > 0 && vals[j] < vals[j-1] {
            let tmp = vals[j];
            vals[j] = vals[j-1];
            vals[j-1] = tmp;
            j -= 1;
        }
        i += 1;
    }
}


pub fn bubble_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    let mut end = vals.len() - 1;
    loop {
        let mut swapped = false;
        let mut i = 0;
        while i < end {
            if vals[i] > vals[i+1] {
                let tmp = vals[i];
                vals[i] = vals[i+1];
                vals[i+1] = tmp;
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
            let mut tmp = vals.split_at(vals.len()/2);
            chunks.push(tmp.0.to_vec());
            chunks.push(tmp.1.to_vec());
        }
        merge_sort(&mut chunks[0]);
        merge_sort(&mut chunks[1]);

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
}
