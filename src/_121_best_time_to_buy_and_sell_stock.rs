use std::cmp;

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut left: usize = 0; // sell
    let mut right: usize = 1; // buy
    let mut max_prof: i32 = 0; // total profit

    while right < prices.len() {
        let cur_prof = prices[right] - prices[left];
        if prices[left] < prices[right] {
            max_prof = cmp::max(cur_prof, max_prof);
        } else {
            left = right;
        }
        right += 1;
    }

    max_prof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_3() {}
}
