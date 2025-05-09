// Constraints:
// 1 <= n <= 2^31 - 1

use std::collections::HashSet;

/// Returns true if sum of digit squares ends in 1 after repeating process multiple times
/// Function reuses sum as next input if result ends not with 1
/// 
/// Example:
/// Input: n = 19
/// Output: true
/// Explanation:
/// 1^2 + 9^2 = 82
/// 8^2 + 2^2 = 68
/// 6^2 + 8^2 = 100
/// 1^2 + 0^2 + 0^2 = 1
/// 
pub fn is_happy(n: i32) -> bool {
    let mut sums = HashSet::new();

    fn get_digits_sq_sum(mut num: i32) -> i32 {
        let mut sum = 0;

        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }

        sum
    }

    let mut current = n;
    while current != 1 {
        // HashSet::insert() returns false if value already existed
        if !sums.insert(current) {
            return false;
        }
        
        current = get_digits_sq_sum(current);
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
            (19, true),
            (2, false),
            (8, false),
            (((2_i64).pow(31) - 1) as i32, false), // overflow test
        ];

        for (n, expected) in cases {
            assert_eq!(
                is_happy(n),
                expected,
            );
        }
    }
}
