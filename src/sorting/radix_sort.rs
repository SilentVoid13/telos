/// naive implementation of the radix sort algorithm.
/// It is not in-place and performs a lot of allocations
pub fn radix_sort_naive(array: &[usize]) -> Vec<usize> {
    let mut digit_idx = 0;
    let mut array = array.to_vec();
    loop {
        let mut buckets = vec![vec![]; 10];
        let mut finished = true;
        for &num in array.iter() {
            let digit = (num / 10_usize.pow(digit_idx)) % 10;
            if digit > 0 {
                finished = false;
            }
            buckets[digit].push(num);
        }
        array = buckets.into_iter().flatten().collect();
        digit_idx += 1;
        if finished {
            return array;
        }
    }
}

pub fn radix_sort_in_place(array: &mut [usize]) {
    // optimal radix base
    let radix = array.len().next_power_of_two();
    // simple way to know when to stop
    let max = *array.iter().max().unwrap_or(&0);
    let mut digit_pow = 1;

    while digit_pow < max {
        let get_digit = |x| (x / digit_pow) % radix;
        let mut digit_count = vec![0usize; radix];

        // counting sort subroutine to determine the number of elements in each bucket
        for &num in array.iter() {
            let digit = get_digit(num);
            digit_count[digit] += 1;
        }
        // create buckets ranges for each digit
        for i in 1..radix {
            digit_count[i] += digit_count[i - 1];
        }
        // place each value in the correct bucket
        for num in array.to_vec().into_iter().rev() {
            let d = get_digit(num);
            digit_count[d] -= 1;
            array[digit_count[d]] = num;
        }
        digit_pow *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::{radix_sort_in_place, radix_sort_naive};

    fn check(mut v: Vec<usize>, res: Vec<usize>) {
        assert_eq!(radix_sort_naive(&v), res);
        radix_sort_in_place(&mut v);
        assert_eq!(v, res);
    }

    #[test]
    fn empty() {
        let v = vec![];
        let res = vec![];
        check(v, res);
    }

    #[test]
    fn descending() {
        let v = vec![201, 127, 64, 37, 24, 4, 1];
        let res = vec![1, 4, 24, 37, 64, 127, 201];
        check(v, res);
    }

    #[test]
    fn ascending() {
        let v = vec![1, 4, 24, 37, 64, 127, 201];
        let res = vec![1, 4, 24, 37, 64, 127, 201];
        check(v, res);
    }

    #[test]
    fn wikipedia() {
        let v = vec![170, 45, 75, 90, 2, 802, 2, 66];
        let res = vec![2, 2, 45, 66, 75, 90, 170, 802];
        check(v, res);
    }
}
