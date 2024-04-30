#[allow(dead_code)]
pub fn counting_bits(n: i32) -> Vec<i32> {
    vec![0; n as usize + 1]
        .into_iter()
        .enumerate()
        .map(|(i, _)| i.count_ones() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(counting_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(counting_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(counting_bits(0), vec![0]);
    }
}
