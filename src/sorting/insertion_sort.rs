pub fn insertion_sort(input: &mut [u32]) {
    for i in 1..input.len() {
        let mut j = i;
        // swap until we find the right place
        while j > 0 && input[j - 1] > input[j] {
            input[j] = input[j - 1];
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut input = vec![6, 5, 4, 3, 2, 1];
        insertion_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![100, 30, 60, 10, 20, 120, 1];
        insertion_sort(&mut input);
        assert_eq!(input, vec![1, 10, 20, 30, 60, 100, 120]);

        let mut input = vec![3, 1, 4, 1, 5, 9, 2, 6];
        insertion_sort(&mut input);
        assert_eq!(input, vec![1, 1, 2, 3, 4, 5, 6, 9]);

        let mut input = vec![];
        insertion_sort(&mut input);
        assert_eq!(input, vec![]);
    }
}
