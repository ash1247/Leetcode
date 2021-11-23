use std::collections::HashMap;

fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let ans = two_sum(nums, target);
    println!("{:#?}", ans);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for index in 0..nums.len() {
        let value = &(target - nums[index]);
        if map.contains_key(value) {
            return vec![*(map.get(value).unwrap()) as i32, index as i32];
        }
        map.insert(nums[index], index);
    }
    return Vec::new();
}

