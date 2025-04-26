// Constraints:
// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].

pub fn roman_to_int(s: String) -> i32 {
    // This works with Roman numbers
    // and supports invalid Roman numbers
    // e.g IIIVM = II + IV + M = 1006
    let mut result: i32 = 0;
    let mut tmp: i16 = 0;

    for c in s.chars() {
        if tmp > 0 {
            if tmp == 1 && (c == 'V' || c == 'X') {
                result -= 2;
            } else if tmp == 10 && (c == 'L' || c == 'C') {
                result -= 20;
            } else if tmp == 100 && (c == 'D' || c == 'M') {
                result -= 200;
            } else {
                tmp = 0;
            }
        }

        match c {
            'I' => {
                result += 1;
                tmp = 1;
            },
            'V' => {
                result += 5;
            },
            'X' => {
                result += 10;
                tmp = 10;
            },
            'L' => {
                result += 50;
            },
            'C' => {
                result += 100;
                tmp = 100;
            },
            'D' => {
                result += 500;
            },
            'M' => {
                result += 1000;
            },
            _ => {
                // incorrect Roman number
                return -1;
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            ("III", 3),
            ("LVIII", 58),
            ("MCMXCIV", 1994),
            ("IIIVM", 1006),
        ];

        for (input, expected) in cases {
            assert_eq!(roman_to_int(input.to_string()), expected, "{:?}", input);
        }
    }
}
