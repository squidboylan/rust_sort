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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut vals = [1, 5, 4, 6, 7, 2, 3];
        insertion_sort(&mut vals);
        assert_eq!(vals, [1, 2, 3, 4, 5, 6, 7]);
    }
}
