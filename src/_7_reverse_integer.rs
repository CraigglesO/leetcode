#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let mut xx = x as i64;
    // figure out the sign and take the absolute value
    let neg: i64 = if xx < 0 { -1 } else { 1 };
    xx = xx.abs();
    // split xx into groups of 10s
    let mut chunks: Vec<i64> = vec![];
    // get each group of 10s
    while xx > 0 {
        chunks.push(xx % 10);
        xx /= 10;
    }
    // reverse our grouping
    chunks.reverse();
    // fold
    let res = neg
        * chunks
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x * 10i64.pow(i as u32));

    if res < i32::MIN as i64 || res > i32::MAX as i64 {
        return 0;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn test_4() {
        assert_eq!(reverse(1534236469), 0);
    }
}
