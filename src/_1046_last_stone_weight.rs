#[allow(dead_code)]
pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
    while stones.len() > 1 {
        stones.sort_by(|a, b| b.cmp(a));
        if stones[0] == stones[1] {
            stones.splice(0..2, vec![]);
        } else {
            stones.splice(0..2, vec![stones[0].abs_diff(stones[1]) as i32]);
        }
    }

    if stones.is_empty() {
        return 0;
    }

    stones[0]
}

// NOTE: The technically correct solution is a BinaryHeap, but this is
// actually faster and less code for this solution.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(last_stone_weight(vec![2, 2]), 0);
    }
}
