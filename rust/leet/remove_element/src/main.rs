pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0; // count of removals 
    let mut current = 0; // current element index we are modifying

    for i in 0..nums.len() {
        if nums[i] != val {
           nums[current] = nums[i];
           current += 1; 
        } else {
            k += 1;
        }
    }

    return (nums.len() - k) as i32;
}

fn main() {
    println!("nothing to test")
}
