#[allow(dead_code)]
pub fn reverse_string(s: &mut [char]) {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut str = "hello".chars().collect::<Vec<char>>();
        let rev = "olleh".chars().collect::<Vec<char>>();
        reverse_string(&mut str);
        assert_eq!(str, rev);
    }

    #[test]
    fn test_2() {
        let mut str = "Hannah".chars().collect::<Vec<char>>();
        let rev = "hannaH".chars().collect::<Vec<char>>();
        reverse_string(&mut str);
        assert_eq!(str, rev);
    }
}
