pub fn bubble_sort(input: &mut [u32]) {
    if input.len() < 2 {
        return;
    }
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..input.len()-1 {
            if input[i] > input[i+1] {
                input.swap(i, i+1);
                swapped = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut input = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![100, 30, 60, 10, 20, 120, 1];
        bubble_sort(&mut input);
        assert_eq!(input, vec![1, 10, 20, 30, 60, 100, 120]);

        let mut input = vec![];
        bubble_sort(&mut input);
        assert_eq!(input, vec![]);
    }
}
