// Problem 13. Jewels and Stones
fn num_jewels_in_stones(jewels: &str, stones: &str) -> Option<i32> {
    let jewel_count = stones
        .chars()
        .filter(|&stone| jewels.contains(stone))
        .count();
    i32::try_from(jewel_count).ok()
}
pub fn solve() {
    let jewels = "aA";
    let stones = "aAAbbbb";
    match num_jewels_in_stones(jewels, stones) {
        Some(count) => println!("Number of jewels in stones: {count}"),
        None => println!("Failed to convert the count value to i32"),
    }
}
