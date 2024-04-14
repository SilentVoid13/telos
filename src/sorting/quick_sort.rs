use std::fmt::Debug;

pub fn quick_sort<E>(range: &mut [E])
where
    E: Ord + Debug,
{
    // there is nothing to do
    if range.len() <= 1 {
        return;
    }

    // pick the pivot as the middle element of the range
    let mut pivot_idx = range.len() / 2;

    // partition the range based on the pivot
    let mut left = 0;
    let mut right = range.len() - 1;
    loop {
        while range[left] < range[pivot_idx] {
            left += 1;
        }
        while range[right] > range[pivot_idx] {
            right -= 1;
        }
        if left >= right {
            break;
        }

        // to prevent infinite loop on multiple pivot value
        if range[left] == range[right] {
            left += 1;
            right -= 1;
        } else {
            // to keep the pivot_idx updated
            if left == pivot_idx {
                pivot_idx = right;
            } else if right == pivot_idx {
                pivot_idx = left;
            }
            // swap the elements
            range.swap(left, right);
        }
    }

    // recurse on the partitions
    quick_sort(&mut range[0..left]);
    quick_sort(&mut range[left..]);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;
    use rand::Rng;

    fn generate_random_vec(n: u32, range_l: i32, range_r: i32) -> Vec<i32> {
        let mut arr = Vec::<i32>::with_capacity(n as usize);
        let mut rng = rand::thread_rng();
        let mut count = n;

        while count > 0 {
            arr.push(rng.gen_range(range_l..range_r + 1));
            count -= 1;
        }

        arr
    }

    fn generate_repeated_elements_vec(n: u32, unique_elements: u8) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let v = rng.gen_range(0..n as i32);
        generate_random_vec(n, v, v + unique_elements as i32)
    }

    fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: PartialOrd,
    {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        quick_sort(&mut res);
        assert_eq!(res, Vec::<u8>::new());
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1000000, 3);
        quick_sort(&mut res);
        assert!(is_sorted(&res));
    }
}
