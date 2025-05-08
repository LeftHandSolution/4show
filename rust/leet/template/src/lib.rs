// Constraints:


// Returns 2 indices of nums where sum is target value
pub fn something(a: i32, b: i32) -> i32 {
   a + b 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            ((1, 2), 3),
        ];

        for ((v1, v2), expected) in cases {
            assert_eq!(
                something(
                    v1,
                    v2,
                ),
                expected,
            );
        }
    }
}
