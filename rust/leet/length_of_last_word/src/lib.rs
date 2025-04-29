// Constraints:
// 1 <= s.length <= 104
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.

pub fn length_of_last_word(s: String) -> i32 {
    let mut current_idx = 0;
    let mut end_idx = -1;
    
    for c in s.chars().rev() {
        let is_alphabetic = c.is_alphabetic();
        
        if is_alphabetic && end_idx == -1 {
            end_idx = current_idx;
        } else if !is_alphabetic && end_idx >= 0 {
            break;
        }
        
        current_idx += 1;
    }
    
    return current_idx - end_idx;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            ("Hello World", 5),
            ("   fly me   to   the moon  ", 4),
            ("uffy is still joyboy", 6),
            ("something", 9)
        ];

        for (input, expected) in cases {
            assert_eq!(length_of_last_word(input.to_string()), expected);
        }
    }
}
