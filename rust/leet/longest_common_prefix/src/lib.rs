// Constraints:
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters if it is non-empty.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    } else if strs.len() == 1 || strs[0].is_empty() {
        return strs[0].clone();
    }

    let bytes = strs[0].as_bytes();
    let mut prefix_len = bytes.len(); // treat as length, not index

    for s in &strs[1..] {
        if s.is_empty() {
            return String::new();
        }

        let s_bytes = s.as_bytes();
        let len = prefix_len.min(s_bytes.len());

        for i in 0..len {
            if bytes[i] != s_bytes[i] {
                prefix_len = i;
                break;
            }
        }

        // If loop ran full length and no mismatch, still update to shorter string
        prefix_len = prefix_len.min(s_bytes.len());

        if prefix_len == 0 {
            return String::new();
        }
    }

    // slicing with length, not inclusive index
    strs[0][..prefix_len].to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            (vec!["flower","flow","flight"], "fl"),
            (vec!["f", "f", "f"], "f"),
            (vec!["abc", "a", "ab"], "a"),
            (vec!["flower", "flower", "flower", "flower"], "flower"),
            (vec!["cannon", "can", "ball"], ""),
            (vec!["dog", "racecar", "car"], ""),
            (vec!["", "b"], ""),
            (vec!["abc", ""], ""),
            (vec!["aaa", "aa", "aaa"], "aa"),
            (vec![], ""),
        ];

        for (input, expected) in cases {
            println!("{:?}", input);
            assert_eq!(
                longest_common_prefix(
                    input
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect()
                ),
                expected,
            );
        }
    }
}
