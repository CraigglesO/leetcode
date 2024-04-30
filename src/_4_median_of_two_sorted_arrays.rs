#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // first concat the vectors
    let mut merged = [&nums1[..], &nums2[..]].concat();
    // sort
    merged.sort();
    // if even number of elements return the average of the two middle elements
    if merged.len() % 2 == 0 {
        (merged[merged.len() / 2] + merged[merged.len() / 2 - 1]) as f64 / 2.0
    } else {
        merged[merged.len() / 2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_3() {
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }
}
