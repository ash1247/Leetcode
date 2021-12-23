use std::collections::HashMap;

fn main() {
    let nums = vec![1, 5, 2, 10];
    let nums1 = vec![87, 68, 91, 86, 58, 63, 43, 98, 6, 40];
    let nums2 = vec![7, 1, 5, 4];
    let ans = maximum_difference_two_pointer(nums1);
    println!("{}", ans);
}

pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let mut i = 1;
    let mut max_diff = -1;
    let mut min_num = nums[0];

    while i < length {
        if nums[i] > min_num {
            max_diff = max_diff.max(nums[i] - min_num);
        }
        min_num = min_num.min(nums[i]);
        i += 1;
    }
    return max_diff;
}

pub fn maximum_difference_two_pointer(nums: Vec<i32>) -> i32 {
    let mut lp = 0;
    let mut max_diff = -1;
    for rp in 1..nums.len() {
        if nums[lp] < nums[rp] {
            max_diff = max_diff.max(nums[rp] - nums[lp])
        } else {
            lp = rp
        }
    }
    return max_diff;
}