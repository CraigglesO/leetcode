#[allow(dead_code)]
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut pos = digits.len() - 1;
    digits[pos] += 1;
    while digits[pos] >= 10 {
        digits[pos] = 0;
        if pos == 0 {
            digits.insert(0, 0);
        } else {
            pos -= 1;
        }
        digits[pos] += 1;
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn test_4() {
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    fn test_5() {
        assert_eq!(plus_one(vec![8, 9, 9, 9]), vec![9, 0, 0, 0]);
    }
}
