// Constraints:
// n == nums.length
// 1 <= n <= 5 * 104
// -10^9 <= nums[i] <= 10^9

use std::collections::HashMap;

// Boyer-Moore Voting Algorithm
pub fn majority_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let target: i32 = nums.len() as i32 / 2 + 1;
    
    let mut candidate: i32 = nums[0];
    let mut count: i32 = 1;

    for i in 1..nums.len() {
        if nums[i] == candidate {
            count += 1;
            
            if count >= target {
                return candidate;
            }
        } else {
            count -= 1;
        }

        if count == 0 {
            candidate = nums[i];
            count = 1;
        }
    }
    
    candidate
}

// Counter (initial version)
pub fn majority_element2(nums: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new(); 

    for i in 0..nums.len() {
        *counts.entry(nums[i]).or_insert(0) += 1;
    } 
    
    counts.into_iter().max_by_key(|e| e.1).unwrap().0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // No testing for edge cases as it's not required for submission
        let cases = vec![
            (vec![3, 3, 4], 3),
            (vec![3, 2, 3], 3),
            (vec![2, 2, 1, 1, 1, 2, 2], 2),
            (vec![1], 1),
        ];

        for (input, expected) in cases {
            assert_eq!(majority_element(input.clone()), expected, "majority_element");
            assert_eq!(majority_element2(input.clone()), expected, "majority_element2");
        }
    }
}
