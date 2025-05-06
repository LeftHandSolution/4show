// Constraints:
// 1 <= haystack.length, needle.length <= 104
// haystack and needle consist of only lowercase English characters.

/// Returns the index of the first occurrence of `needle` in `haystack`, or -1 if not found.
/// Will not correctly work in multibyte characters scenario, but it's fine for current constraints
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() > haystack.len() {
        return -1;
    }
    
    // We use bytes instead of chars() to avoid utf8 decoding
    // so it can actually handle very long strings efficiently
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let mut idx = 0;
    
    while idx <= haystack_bytes.len() - needle_bytes.len() {
        if &haystack_bytes[idx..(idx + needle_bytes.len())] == needle_bytes {
            return idx as i32;
        }

        idx += 1;
    }
    
    // Return -1 if needle not found
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![
            (("sadbutsad", "sad"),  0),
            (("leetcode", "leeto"), -1),
            (("leet", "leeto"), -1),
            (("hello", "ll"), 2),
            (("lietuvių žodžiai", "žodž"), 10),
            (("a", "a"), 0),
        ];

        for ((haystack, needle), expected) in cases {
            assert_eq!(
                str_str(
                    haystack.to_string(),
                    needle.to_string()
                ),
                expected,
                "{}, {}", haystack, needle
            );
        }
    }
}
