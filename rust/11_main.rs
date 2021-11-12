use std::cmp;

fn main() {

    let vec = vec![1,8,6,2,5,4,8,3,7];
    let ans = max_area(vec);
    println!("{}", ans);
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0; 
    let size = height.len() - 1;
    let mut left_pointer = 0;
    let mut right_pointer = size;
    while right_pointer > left_pointer {
        let res = ((right_pointer - left_pointer) as i32) * cmp::min(height[left_pointer], height[right_pointer]);
        area = cmp::max(area, res);

        if height[left_pointer] < height[right_pointer] {
            left_pointer += 1;
        } else {
            right_pointer -= 1;
        }
    }
    return area;
}

pub fn max_area_brute(height: Vec<i32>) -> i32 {
    let mut area = 0; 
    let size = height.len();
    for left_index in 0..size {
        for right_index in (left_index + 1)..size {
            let res = ((right_index - left_index) as i32) * cmp::min(height[left_index], height[right_index]);
            area = cmp::max(area, res);
        }
    }
    return area;
}
