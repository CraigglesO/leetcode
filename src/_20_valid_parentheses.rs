#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        let last = *stack.last().unwrap_or(&' ');
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' {
            if (last == '(' && c == ')') || (last == '[' && c == ']') || (last == '{' && c == '}') {
                stack.pop();
            } else {
                return false;
            }
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!is_valid("(]".to_string()));
    }
}
