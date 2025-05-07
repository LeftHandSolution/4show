// Constraints:
// 1 <= ransomNote.length, magazine.length <= 105
// ransom_note and magazine consist of lowercase English letters.

use std::collections::HashMap;

/// Returns if ransom note can be constructed from letters in magazine
/// by using letters only once.
/// For speed improvement we can use array instead of hashmap
/// as we have only lowercase English letters
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut available_letters = HashMap::<char, i32>::new();
    
    for c in magazine.chars() {
        available_letters
            .entry(c)
            .and_modify(|e| *e += 1)
            .or_insert(1);

    }

    for c in ransom_note.chars() {
        available_letters
            .entry(c)
            .and_modify(|e| *e -= 1)
            .or_insert(-1);
        
        if available_letters[&c] < 0 {
            return false;
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
            (("a", "b"), false),
            (("aa", "ab"), false),
            (("aa", "aab"), true),
            (("aa", "baa"), true),
            (("aa", ""), false),
        ];

        for ((ransom_note, magazine), expected) in cases {
            assert_eq!(
                can_construct(
                    ransom_note.to_string(),
                    magazine.to_string(),
                ),
                expected,
                "{} # {}", ransom_note, magazine
            );
        }
    }
}
