// Constraints:
// 1 <= s.length <= 5 * 104
// t.length == s.length
// s and t consist of any valid ascii character.

use std::collections::HashMap;

/// Returns if string letters can be replaced by letters from t while preserving order
/// Examples
/// s: "egg" and t: "add" -> true, because e->a g->d
/// s: "foo" and t: "bar" -> false, because 'oo' would need to be mapped to 'ar'
/// 
/// Would produce incorrect results with multibyte characters
///
pub fn is_isomorphic(s: String, t: String) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut stmap = HashMap::<u8, u8>::new();
    let mut tsmap = HashMap::<u8, u8>::new();


    for idx in 0..s_bytes.len() {
        let s_byte = s_bytes[idx];
        let t_byte = t_bytes[idx];
        
        if  *stmap.entry(s_byte).or_insert(t_byte) != t_byte {
            return false;
        }

        if  *tsmap.entry(t_byte).or_insert(s_byte) != s_byte {
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
            (("egg", "add"), true),
            (("foo", "bar"), false),
            (("ababbc", "zxzxxc"), true),
            (("badc", "baba"), false),
        ];

        for ((s, t), expected) in cases {
            assert_eq!(
                is_isomorphic(
                    s.to_string(),
                    t.to_string(),
                ),
                expected,
                "{} # {}", s, t
            );
        }
    }
}
