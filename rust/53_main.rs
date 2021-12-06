
fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let nums1 = vec![5,4,-1,7,8];
    let ans = max_sub_array_kandane(nums1);
    println!("{}", ans);
}

// There might be some follow up questions like how to get the length of max
// subarray, start and end index of that array and elements of that array.
pub fn max_sub_array_kandane(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut sum = 0;
    let mut start_index = 0;
    let mut end_index = 0;
    
    for index in 0..nums.len() {
        sum += nums[index];
        max_sum = sum.max(max_sum);
        if sum < 0 {
            start_index = index + 1;
            sum = 0;
        }
        if sum == max_sum {
            end_index = index;
        }
    }
    println!("{} {}", start_index, end_index);
    let length_of_subarray = end_index - start_index + 1;
    println!("{}", length_of_subarray);

    return max_sum;
}

pub fn max_sub_array_iterative(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_max_sum = nums[0];
    for index in 1..nums.len() {
        current_max_sum = nums[index].max(current_max_sum + nums[index]);
        max_sum = max_sum.max(current_max_sum)
    }
    return max_sum;
}


fn max_sub_array_recursive(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let mut max_sum = 0;
    max_sub_array(&nums, length, &mut max_sum);
    return max_sum;
}

pub fn max_sub_array(nums: &Vec<i32>, length: usize, max_sum: &mut i32) -> i32 {
    if length == 1 {
        return nums[0];
    }
    let current_max_sum = nums[length - 1].max(max_sub_array(&nums, length - 1, max_sum) + nums[length - 1]);
    *max_sum = current_max_sum.max(*max_sum);
    return current_max_sum;
}
