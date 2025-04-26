// Constraints:
// 1 <= nums.length <= 3 * 104
// -104 <= nums[i] <= 104
// nums is sorted in non-decreasing order.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut insert_idx: usize = 2;

    for i in 2..nums.len() {
        if nums[i] != nums[insert_idx - 2] {
            nums[insert_idx] = nums[i];
            insert_idx += 1;
        }
    }

    return insert_idx as i32;
}

fn main() {
    let cases: Vec<(Vec<i32>, Vec<i32>, i32)> = vec![
        (vec![1, 1, 2], vec![1, 1, 2], 3),
        (vec![1, 1, 1, 2], vec![1, 1, 2], 3),
        (vec![1, 1, 1, 2, 2, 3, 3, 3], vec![1, 1, 2, 2, 3, 3], 6),
    ];

    for (mut nums, expected, count) in cases {
        println!("{:?}", nums);
        
        let answer = remove_duplicates(&mut nums);

        assert_eq!(answer, count);
        assert_eq!(nums.split_at(answer as usize).0, expected);
    }
}