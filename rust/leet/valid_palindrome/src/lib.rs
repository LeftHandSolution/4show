// Constraints:
// 1 <= s.length <= 2 * 105
// s consists only of printable ASCII characters.

/// Returns if string is a valid polindrome after removing all non-alphanumeric characters.
/// Doesn't support multibyte characters.
pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let bytes = s.as_bytes();
    let mut start_idx = 0;
    let mut end_idx = s.len() - 1;
    
    loop {
        if start_idx >= end_idx {
            break;
        }

        // For efficiency to avoid bound checking in conditions
        let start = bytes[start_idx];
        let end = bytes[end_idx];

        if !start.is_ascii_alphanumeric() {
            start_idx += 1;
            continue;
        }

        if !end.is_ascii_alphanumeric() {
            end_idx -= 1;
            continue;
        }
        
        if start.to_ascii_lowercase() == end.to_ascii_lowercase() {
            start_idx += 1;
            end_idx -= 1;
            continue;
        } else {
            // Return false if mismatch is found
            return false;
        }
        
    }

   // Return true as we did not find any mismatches 
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            ("race a car", false),
            ("A man, a plan, a canal: Panama", true),
            (" ", true), // sample from exercise...
            (" sas ", true),
            (" s as ", true),
            ("s as ", true),
            (" saas ", true),
            ("saas ", true),
        ];

        for (input, expected) in cases {
            assert_eq!(is_palindrome(
                input.to_string()),
                expected,
                "{}", input
            );
        }
    }
}
