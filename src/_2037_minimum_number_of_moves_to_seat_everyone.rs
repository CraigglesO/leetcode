#[allow(dead_code)]
pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort();
    students.sort();
    seats
        .iter()
        .zip(students.iter())
        .map(|(s, st)| (s - st).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }
}
