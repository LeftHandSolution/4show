// Constraints:
// 0 <= s.length <= 100
// 0 <= t.length <= 104
// s and t consist only of lowercase English letters.

// Returns true if a string s can be formed from string t
// by deleting some characters without disturbing relative positions.
pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut s_found = 0;
    let mut t_checked = 0;
    
    'outer: for s_idx in 0..s_bytes.len() {
        for t_idx in t_checked..t_bytes.len() {
            if s_bytes[s_idx] == t_bytes[t_idx] {
                s_found += 1;
                t_checked += 1;
                if s_found == s_bytes.len() {
                    return true;
                } else {
                    continue 'outer;
                }
            } else {
                t_checked += 1;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            (("abc", "ahbgdc"), true),
            (("axc", "ahbgdc"), false),
            (("", "aaa"), true),
            (("bbb", ""), false),
        ];

        for ((s, t), expected) in cases {
            assert_eq!(
                is_subsequence(
                    s.to_string(),
                    t.to_string(),
                ),
                expected,
                "s: {}, t:{}", s, t
            );
        }
    }
}
