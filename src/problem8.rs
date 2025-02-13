// Problem 8. Number of Good Pairs
use std::collections::HashMap;
fn num_good_pairs(nums: &Vec<i32>) -> i32 {
    let mut good_dict = HashMap::new();
    for val in nums {
        good_dict
            .entry(val)
            .and_modify(|x| *x = *x + *x + 1)
            .or_insert(0);
    }
    good_dict.values().sum()
}

pub fn solve() {
    // Solution for Problem 8
    let nums: Vec<i32> = vec![1, 2, 3, 1, 1, 3];
    let good_pairs = num_good_pairs(&nums);
    println!("There are {good_pairs} good pairs in the array!");
}
