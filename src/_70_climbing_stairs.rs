#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 1;

    for i in 1..=n as usize {
        if i == 1 {
            dp[i] = 1;
        } else {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
    }

    dp[dp.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(3), 3);
    }
}
