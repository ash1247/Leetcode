use std::collections::HashMap;

fn main() {
    let nums = vec![1, 1, 1, 6, 1, 1, 1];
    let nums1 = vec![5, 4, -1, 7, 8];
    let nums2 = vec![1, 2, 3, 1];
    let ans = max_distance(nums);
    println!("{}", ans);
}

pub fn max_distance(colors: Vec<i32>) -> i32 {
    let length = colors.len();
    let mut i = 0;
    let mut j = length - 1;

    while colors[0] == colors[j] {
        j -= 1
    }
    while colors[length - 1] == colors[i] {
        i += 1
    }
    return j.max(length - 1 - i) as i32;
}
