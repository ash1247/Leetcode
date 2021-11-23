use std::collections::HashSet;
use std::cmp;

fn main() {
    let s = "a()*+,-./:;<=>?@[\\]^_`{|}~ abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ
    0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ
    RSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ abcdefghijklmnopqrstuvwxyzABCDEFGH
    IJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:".to_string();
    let ans = length_of_longest_substring_dictionary_and_sliding_pointer(s);
    println!("{:#?}", ans);
}

pub fn length_of_longest_substring_dictionary_and_sliding_pointer(s: String) -> i32 {
    let mut vector: Vec<i32> = vec![0; 128];
    let mut left_pointer: i32 = 0;
    let mut result = 0;
    for (right_pointer, ch) in s.char_indices().map( |(index, c)| (index as i32, c as usize) ) {
        left_pointer = vector[ch].max(left_pointer);
        result = result.max(right_pointer - left_pointer + 1);
        vector[ch] = right_pointer + 1;
    } 
    return result;
}

pub fn length_of_longest_substring_sliding_pointer(s: String) -> i32 {
    let mut left_pointer = 0;
    let mut right_pointer = 0;
    let mut result = 0;
    let mut set: HashSet<char> = HashSet::new();
    while right_pointer < s.len() {
        if set.contains(&s.chars().nth(right_pointer).unwrap()) {
            result = cmp::max(result, right_pointer - left_pointer);
            set.remove(&s.chars().nth(left_pointer).unwrap());
            left_pointer += 1;
        } else {
            set.insert(s.chars().nth(right_pointer).unwrap());
            right_pointer += 1;
        }
    }
    result = cmp::max(result, right_pointer - left_pointer);
    return result as i32;
}