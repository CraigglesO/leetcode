use std::collections::HashMap;

// NOTE: The "best" solution on leetcode is just sorting the vector
// and finding the longest run. I don't agree, as hashes in Rust
// are extremely slow until they are optimized via a correct build.

// ALSO: sorting the vector is an O(n log n) operation, and that's
// technically not allowed by the problem.

#[allow(unused)]
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;

    for n in nums {
        if map.contains_key(&n) {
            continue;
        }
        let left = map.get(&(n - 1)).copied().unwrap_or(0);
        let right = map.get(&(n + 1)).copied().unwrap_or(0);
        let sum = left + right + 1;
        max = max.max(sum);

        // update
        map.insert(n, sum);
        // Only update the boundary values if they exist
        if left > 0 {
            map.insert(n - left, sum);
        }
        if right > 0 {
            map.insert(n + right, sum);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
