
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }

    if m == 0 {
        nums2.clone_into(nums1);
        return;
    }

    let mut end_index = m + n - 1;
    let mut num1_index = m - 1;
    let mut num2_index = n - 1;
    
    while num2_index >= 0 {
        if num1_index >= 0 && nums1[num1_index as usize] >= nums2[num2_index as usize] {
            nums1[end_index as usize] = nums1[num1_index as usize];
            num1_index -= 1;
        } else {
            nums1[end_index as usize] = nums2[num2_index as usize];
            num2_index -= 1;
        }

        end_index -= 1;        
    }
}

fn main() {
    let cases = vec![
        (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
        (vec![1, 2, 4, 6, 0, 0, 0, 0], 4, vec![3, 4, 4, 7], 4),
        (vec![2, 0], 1, vec![1], 1),
        (vec![4, 5, 6, 0, 0, 0], 3, vec![1, 2, 3],3 ),
        (vec![1], 1, vec![], 0),
        (vec![0], 0, vec![1], 1),
    ];

    for (mut nums1, m, mut nums2, n) in cases {
        println!("{:?}", nums1);
        println!("{:?}", nums2);
        merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
        println!("####################");
    }
}
