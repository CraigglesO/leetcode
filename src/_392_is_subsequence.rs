#[allow(dead_code)]
pub fn is_subsequence(s: String, t: String) -> bool {
    let schars = s.chars().collect::<Vec<_>>();
    let tchars = t.chars().collect::<Vec<_>>();
    let mut pos = 0;

    for i in 0..t.len() {
        if pos < s.len() && schars[pos] == tchars[i] {
            pos += 1;
        }
    }

    pos == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false,
        );
    }
}
