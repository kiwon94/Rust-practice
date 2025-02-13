// Problem 6. Shuffle the Array

fn shuffle(nums: &[i32]) -> Vec<i32> {
    let n = nums.len() / 2;
    nums[..n]
        .iter()
        .zip(&nums[n..])
        .flat_map(|(&x, &y)| vec![x, y])
        .collect()
}

pub fn solve() {
    // Solution for Problem 6
    let nums: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    let result = shuffle(&nums);
    println!("Shuffled array: {result:?}");
}
