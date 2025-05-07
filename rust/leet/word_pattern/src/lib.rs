// Constraints:
// 1 <= pattern.length <= 300
// pattern contains only lower-case English letters.
// 1 <= s.length <= 3000
// s contains only lowercase English letters and spaces ' '.
// s does not contain any leading or trailing spaces.
// All the words in s are separated by a single space.

use std::collections::HashMap;

/// Returns if there is bijection between letters and non-empty words
/// Example: pattern: "aba", s: "dog cat dog -> true, because a->dog, b->cat
pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut lwmap =  HashMap::<char, String>::new();
    let mut wlmap =  HashMap::<String, char>::new();

    let mut siter = s.chars();
    
    for pc in pattern.chars() {
        let mut word = String::new();
        loop {
            match siter.next() {
                Some(' ') => break,
                Some(wc) => word.push(wc),
                None => break,
            }
        }
        
        if word.is_empty() {
            return false;
        }
        
        if lwmap.entry(pc).or_insert(word.clone()) != &word {
            return false;
        }
        
        if wlmap.entry(word).or_insert(pc) != &pc {
            return false;
        }
    }

    // Catch if we have any extra letters remaining
    if siter.any(|c| c != ' ') {
        return false;
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
            (("abba", "dog cat cat dog "), true),
            (("abba", "dog cat cat fish"), false),
            (("aaaa", "dog cat cat dog"), false),
            (("aaa", "aa aa aa aa"), false),
        ];

        for ((s1, s2), expected) in cases {
            assert_eq!(
                word_pattern(
                    s1.to_string(),
                    s2.to_string(),
                ),
                expected,
                "{} # {}", s1, s2
            );
        }
    }
}
