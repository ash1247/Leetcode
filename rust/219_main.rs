use std::collections::HashMap;

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let nums1 = vec![5,4,-1,7,8];
    let nums2 = vec![1,2,3,1];
    let k = 3;
    let ans = contains_nearby_duplicate(nums2, k);
    println!("{}", ans);
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if let Some(i_th_value) = map.insert(*num, index as i32) {
            if (i_th_value - index as i32).abs() <= k {
                return true;
            }
        }
    }
    return false;
}