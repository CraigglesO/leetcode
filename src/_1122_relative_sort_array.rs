use std::collections::HashMap;

#[allow(dead_code)]
pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut res: Vec<i32> = vec![];
    // items not in arr2 are added here then sorted and appended to the result
    let mut end: Vec<i32> = vec![];

    arr2.into_iter().enumerate().for_each(|(i, item)| {
        map.insert(item, i);
    });

    arr1.into_iter().for_each(|item| {
        if map.contains_key(&item) {
            res.push(item);
        } else {
            end.push(item);
        }
    });

    // now sort res and end
    end.sort();
    res.sort_by(|a, b| map.get(a).unwrap_or(&0).cmp(map.get(b).unwrap_or(&0)));
    res.extend(end);

    res
}

// NOTE: There are faster solutions online that don't use Hashes but they are ugly as sin and don't scale.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }
}
