// Constraints:
// 1 <= s.length, t.length <= 5 * 104
// s and t consist of lowercase English letters.

use std::collections::HashMap;

/// Returns if t is an anagram of s
pub fn is_anagram(s: String, t: String) -> bool {
    if s.is_empty() || t.is_empty() || s.len() != t.len() {
        return false;
    }
    
    let mut counter = HashMap::new();
    
    s.chars().for_each(|ch| *counter.entry(ch).or_insert(0) += 1);
    
    for ch in t.chars() {
        match counter.get_mut(&ch) {
            Some(el) => {
                *el -= 1;
                if *el < 0 {
                    return false;
                }
            },
            None => return false,            
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            (("anagram", "nagaram"), true),
            (("ana", "aana"), false),
            (("rat", "car"), false),
            (("aa", "aaa"), false),
            (("a", "a"), true),
            (("", "a"), false),
            (("a", ""), false),
        ];

        for ((s1, s2), expected) in cases {
            assert_eq!(
                is_anagram(
                    s1.to_string(),
                    s2.to_string(),
                ),
                expected,
                "{} # {}", s1, s2
            );
        }
    }
}
