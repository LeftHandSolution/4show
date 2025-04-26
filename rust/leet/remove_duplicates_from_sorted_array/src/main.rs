pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev: i32 = -101; // it's fine as constraints say smallest num is -100
    let mut removed: i32 = 0;

    for i in 0..nums.len() {
        if nums[i] != prev {
            prev = nums[i];
            nums[i - removed as usize] = nums[i];
        } else {
            removed += 1;
        }
    }

    return nums.len() as i32 - removed;
}

fn main() {
    let cases: Vec<(Vec<i32>, Vec<i32>, i32)> = vec![
        (vec![1, 1, 2], vec![1, 2], 2),
        (vec![1, 1, 1, 2], vec![1, 2], 2),
        (vec![1, 1, 1, 2, 2, 3, 3], vec![1, 2, 3], 3),
        (vec![], vec![], 0)
    ];

    for (mut nums, expected, count) in cases {
        println!("{:?}", nums);
        
        let answer = remove_duplicates(&mut nums);

        assert_eq!(answer, count);
        assert_eq!(nums.split_at(answer as usize).0, expected);
    }
}
