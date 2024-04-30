use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen = HashMap::new();
    let mut left = 0;
    let mut res = 0;

    for (right, char) in s.chars().enumerate() {
        if let Some(&index) = seen.get(&char) {
            if index >= left {
                left = index + 1;
            }
        }

        res = res.max(right - left + 1);
        seen.insert(char, right);
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(length_of_longest_substring("abbbcabcbb".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("pwwwkew".to_string()), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }
}
