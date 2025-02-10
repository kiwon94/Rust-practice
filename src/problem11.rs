// Problem 11. How Many Numbers Are Smaller Than the Current Number
use std::collections::HashMap;
fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut sorted_nums: Vec<i32> = nums.to_vec();
    sorted_nums.sort_unstable();
    let mut rank_map = HashMap::new();
    for (idx, &val) in sorted_nums.iter().enumerate() {
        rank_map
            .entry(val)
            .or_insert(i32::try_from(idx).expect("Index is not a valid i32"));
    }
    let result: Vec<i32> = nums
        .iter()
        .map(|&x| *rank_map.get(&x).expect("The value must  exist in the map"))
        .collect();

    result
}

pub fn solve() {
    let nums = vec![8, 1, 2, 2, 3];
    let result = smaller_numbers_than_current(&nums);
    println!("Input array is {nums:?}, and smaller numbers than current is {result:?}");
}
