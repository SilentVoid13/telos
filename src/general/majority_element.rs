/// Naive solution
/// Iterates through the ballot box and counts the number of votes for each candidate.
/// Uses O(n^2) time and O(1) space.
pub fn naive(ballot_box: Vec<u32>) -> Option<u32> {
    for cur_candidate in ballot_box.iter() {
        let mut count = 0;
        for vote in ballot_box.iter() {
            if vote == cur_candidate {
                count += 1;
            }
        }
        if count > ballot_box.len() / 2 {
            return Some(*cur_candidate);
        }
    }
    None
}

/// Divide and conquer solution
/// Uses O(nlogn) time and O(logn) space.
pub fn divide_and_conquer(ballot_box: &Vec<u32>, left: usize, right: usize) -> Option<u32> {
    if left == right - 1 {
        return Some(ballot_box[left]);
    }

    let middle = (left + right) / 2;
    let left_majority = divide_and_conquer(ballot_box, left, middle);
    let right_majority = divide_and_conquer(ballot_box, middle, right);

    if left_majority.is_none() && right_majority.is_none() {
        return None;
    }
    if left_majority == right_majority {
        return left_majority;
    }
    let total = right - left;
    if left_majority.is_none() {
        let right_majority = right_majority.unwrap();
        let right_freq = ballot_box[left..right]
            .iter()
            .filter(|&x| *x == right_majority)
            .count();
        if right_freq > total / 2 {
            return Some(right_majority);
        }
        return None;
    }
    if right_majority.is_none() {
        let left_majority = left_majority.unwrap();
        let left_freq = ballot_box[left..right]
            .iter()
            .filter(|&x| *x == left_majority)
            .count();
        if left_freq > total / 2 {
            return Some(left_majority);
        }
        return None;
    }
    None
}

/// Booyer-Moore solution
/// Uses O(n) time and O(1) space.
pub fn booyer_moore(ballot_box: Vec<u32>) -> Option<u32> {
    let mut count = 0;
    let mut candidate = 0;
    for vote in ballot_box.iter() {
        if count == 0 {
            candidate = *vote;
        }
        if *vote == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    // This only works if there is a majority element.
    // A second pass is needed to verify that the candidate is indeed the majority element.
    let total = ballot_box.len();
    let candidate_freq = ballot_box.iter().filter(|&x| *x == candidate).count();
    if candidate_freq <= total / 2 {
        return None;
    }
    return Some(candidate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_naive() {
        let ballot_box = vec![4, 1, 3, 4, 2, 4, 4, 3, 4];
        assert_eq!(naive(ballot_box), Some(4));
    }

    #[test]
    pub fn test_divide_and_conquer() {
        let ballot_box = vec![4, 1, 3, 4, 2, 4, 4, 3, 4];
        assert_eq!(
            divide_and_conquer(&ballot_box, 0, ballot_box.len()),
            Some(4)
        );
    }

    #[test]
    pub fn test_booyer_moore() {
        let ballot_box = vec![4, 1, 3, 4, 2, 4, 4, 3, 4];
        assert_eq!(booyer_moore(ballot_box), Some(4));
    }
}
