use std::collections::HashMap;

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut res: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for str in strs {
        let key = build_key(&str);
        res.entry(key).or_insert(vec![]).push(str);
    }

    res.into_values().collect()
}

fn build_key(s: &str) -> [u8; 26] {
    let mut key = [0; 26];

    for c in s.chars() {
        key[c as usize - 97] += 1;
    }

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut grouped_res = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        grouped_res.sort_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(
            grouped_res,
            vec![vec!["bat"], vec!["tan", "nat"], vec!["eat", "tea", "ate"]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(group_anagrams(vec!["".to_string()]), vec![vec![""]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(group_anagrams(vec!["a".to_string()]), vec![vec!["a"]]);
    }
}
