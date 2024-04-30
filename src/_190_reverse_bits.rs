#[allow(dead_code)]
pub fn reverse_bits(mut x: u32) -> u32 {
    let mut result = 0;

    for i in 0..32 {
        result += (x & 1) << (31 - i);
        x >>= 1;
    }

    result
}

// NOTE: In Rust you can just do `x.reverse_bits()`, but that doesn't analyze the situation.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse_bits(43261596), 964176192);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_bits(4294967293), 3221225471);
    }

    #[test]
    fn test_3() {}
}
