use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let nums = vec![1,1,1,3,3,4,3,2,4,2];
    let ans = contains_duplicate_sort(nums);
    println!("{}", ans);
}

pub fn contains_duplicate_sort(nums: Vec<i32>) -> bool {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();
    for index in 1..sorted_nums.len() {
        if sorted_nums[index - 1] == sorted_nums[index] {
            return true;
        }
    }
    return false;
}

pub fn contains_duplicate_set(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    return false;
}

pub fn contains_duplicate_hash_map(nums: Vec<i32>) -> bool {
    let mut hash_map: HashMap<i32, bool> = HashMap::new();
    for num in nums {
        if hash_map.contains_key(&num) {
            return true
        } 
        hash_map.insert(num, true);
    }
    return false;
}

