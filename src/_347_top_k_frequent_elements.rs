use std::collections::HashMap;

#[allow(dead_code)]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|x| {
        *map.entry(*x).or_insert(0) += 1;
    });

    let mut sort: Vec<(i32, i32)> = map.into_iter().collect();
    sort.sort_by(|a, b| b.1.cmp(&a.1));

    sort[0..k as usize].iter().map(|x| x.0).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
