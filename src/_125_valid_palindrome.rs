#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    // use a regex that cleans up the string to only be numbers and letters
    let sanitized: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let left: Vec<char> = sanitized.chars().take(sanitized.len() / 2).collect();
    let right: Vec<char> = sanitized.chars().rev().take(sanitized.len() / 2).collect();

    left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!is_palindrome("race a car".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(is_palindrome(" ".to_string()));
    }
}
