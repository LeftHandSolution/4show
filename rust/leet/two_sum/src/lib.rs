// Constraints:
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists

use std::collections::HashMap;

// Returns 2 indices of nums where sum is target value
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        let diff = target - num;

        if let Some(&other_idx) = indices.get(&diff) {
            return vec![other_idx, idx as i32];
        }
        
        indices.insert(*num, idx as i32);
    }

    panic!("Impossible to achieve target value with provided input!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            ((vec![2, 7, 11, 15], 9), vec![0, 1]),
            ((vec![3, 2, 4], 6), vec![1, 2]),
            ((vec![3, 3], 6), vec![0, 1]),
        ];

        for ((nums, target), expected) in cases {
            assert_eq!(
                two_sum(
                    nums,
                    target,
                ),
                expected,
            );
        }
    }
}
