use std::collections::HashMap;

fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let ans = two_sum_two_pointer(nums, target);
    println!("{:#?}", ans);
}

pub fn two_sum_two_pointer(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_pointer = 0;
    let mut right_pointer = nums.len() - 1;
    while left_pointer < right_pointer {
        if nums[left_pointer] + nums[right_pointer] < target {
            left_pointer += 1;
        } else if nums[left_pointer] + nums[right_pointer] > target {
            right_pointer -= 1;
        } else {
            return vec![(left_pointer + 1) as i32, (right_pointer + 1) as i32];
        }
    }
    return Vec::new();
}

pub fn two_sum_with_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for index in 0..nums.len() {
        let value = &(target - nums[index]);
        if map.contains_key(value) {
            return vec![*(map.get(value).unwrap()) as i32, (index + 1) as i32];
        }
        map.insert(nums[index], index + 1);
    }
    return Vec::new();
}

