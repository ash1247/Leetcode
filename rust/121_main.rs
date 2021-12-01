use std::collections::HashMap;

fn main() {
    let prices = vec![7,1,5,3,6,4];
    let ans = max_profit_two_pointer(prices);
    println!("{}", ans);
}

pub fn max_profit_two_pointer(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut left_pointer = 0;
    for right_pointer in 0..prices.len() {
        if prices[left_pointer] < prices[right_pointer] {
            let profit = prices[right_pointer] - prices[left_pointer];
            max_profit = profit.max(max_profit);
        } else {
            left_pointer = right_pointer;
        }
    }
    return max_profit;
}

pub fn max_profit_brute_force(prices: Vec<i32>) -> i32 {
    let mut ans = 0;
    for index1 in 0..prices.len() {
        let mut temp_ans = 0;
        for index2 in (index1 + 1)..prices.len() {
            let value = prices[index2] - prices[index1];
            temp_ans = temp_ans.max(value);
        }
        ans = ans.max(temp_ans);
    }
    return ans;
}