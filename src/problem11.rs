// Problem 11. How Many Numbers Are Smaller Than the Current Number
fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|&x| nums.iter().filter(|&&y| y < x).count())
        .collect()
}

pub fn solve() {
    let nums = vec![8, 1, 2, 2, 3];
    let result = smaller_numbers_than_current(&nums);
    println!("Input array is {nums:?}, and smaller numbers than current is {result:?}");
}
