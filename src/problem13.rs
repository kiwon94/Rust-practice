// Problem 12. Defanging an IP Address
use std::collections::HashSet;

fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let jewel_set: HashSet<char> = jewels.chars().collect();
    let mut jewel_cnt = 0;

    for stone in stones.chars() {
        if jewel_set.contains(&stone) {
            jewel_cnt += 1;
        }
    }

    jewel_cnt
}

pub fn solve() {
    let jewels = "aA";
    let stones = "aAAbbbb";
    let result = num_jewels_in_stones(jewels, stones);
    println!("Number of jewels in stones: {result}");
}
