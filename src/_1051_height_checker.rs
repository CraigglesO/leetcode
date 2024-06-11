use std::vec::Vec;

#[allow(dead_code)]
pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.clone();
    sorted.sort();
    sorted.into_iter().enumerate().fold(
        0,
        |acc, (idx, n)| {
            if heights[idx] != n {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            height_checker(vec![
                10, 6, 6, 10, 10, 9, 8, 8, 3, 3, 8, 2, 1, 5, 1, 9, 5, 2, 7, 4, 7, 7
            ]),
            22
        );
    }
}
