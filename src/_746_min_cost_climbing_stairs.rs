use std::cmp::min;

#[allow(dead_code)]
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut cost = cost;
    for i in 2..cost.len() {
        cost[i] += min(cost[i - 1], cost[i - 2]);
    }
    min(cost[cost.len() - 1], cost[cost.len() - 2])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }
}
