use std::cmp::Ordering;

#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![];

    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() - 2 {
        if nums[i] > 0 {
            break;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match sum.cmp(&0) {
                Ordering::Equal => {
                    out.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
                Ordering::Less => {
                    j += 1;
                }
                Ordering::Greater => {
                    k -= 1;
                }
            }
        }
    }

    out
}

// NOTE: I found the simpler solution was to just use a HashSet. But this solution is more performant.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn test_3() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
