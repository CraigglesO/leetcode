#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    s.len() == t.len() && build_key(&s) == build_key(&t)
}

fn build_key(s: &str) -> [u16; 26] {
    let mut key = [0; 26];

    s.as_bytes()
        .iter()
        .for_each(|c| key[(c - b'a') as usize] += 1);

    key
}

// create a function that adds two i32

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!(is_anagram("rat".to_string(), "car".to_string())));
    }

    #[test]
    fn test_3() {}
}
