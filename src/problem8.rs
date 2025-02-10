use std::collections::HashMap;
fn combination_2(n: i32) -> i32 {
    n * (n - 1) / 2
}
fn num_good_pairs(nums: &Vec<i32>) -> i32 {
    let mut good_dict = HashMap::new();
    for val in nums {
        *good_dict.entry(val).or_insert(0) += 1;
    }

    let sum: i32 = good_dict
        .values()
        .filter(|&&x| x > 1)
        .map(|&x| combination_2(x))
        .sum();
    sum
}

pub fn solve() {
    // Solution for Problem 8
    let nums: Vec<i32> = vec![1, 2, 3, 1, 1, 3];
    let good_pairs = num_good_pairs(&nums);
    println!("There are {good_pairs} good pairs in the array!");
}
