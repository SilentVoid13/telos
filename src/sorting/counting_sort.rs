pub fn counting_sort(input: &mut [u32], maxval: usize) {
    let mut occurences = vec![0; maxval + 1];

    // Count occurences of each value
    for &val in input.iter() {
        occurences[val as usize] += 1;
    }

    // Place each value `occ` times
    let mut i = 0;
    for (occ_i, &occ) in occurences.iter().enumerate() {
        for _ in 0..occ {
            input[i] = occ_i as u32;
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut input = vec![6, 5, 4, 3, 2, 1];
        counting_sort(&mut input, 6);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![1, 2, 3, 4, 5, 6];
        counting_sort(&mut input, 6);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6]);

        let mut input = vec![100, 30, 60, 10, 20, 120, 1];
        counting_sort(&mut input, 120);
        assert_eq!(input, vec![1, 10, 20, 30, 60, 100, 120]);
    }
}
