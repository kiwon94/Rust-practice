// Problem 6. Shuffle the Array

fn shuffle(nums: &[i32], n: i32) -> Result<Vec<i32>, String> {
    let n_usize: usize = match n.try_into() {
        Ok(n_usize) => n_usize,
        Err(_) => {
            return Err("n must be a non-negative integer".to_string());
        }
    };

    let left_half: &[i32] = &nums[..(n_usize)];
    let right_half: &[i32] = &nums[(n_usize)..];
    let mut shuffle_nums: Vec<i32> = Vec::new();
    let zip: Vec<(&i32, &i32)> = left_half.iter().zip(right_half.iter()).collect();
    for (l_num, r_num) in zip {
        shuffle_nums.push(*l_num);
        shuffle_nums.push(*r_num);
    }
    Ok(shuffle_nums)
}

pub fn solve() {
    // Solution for Problem 6
    let nums: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    let n: i32 = 3;
    match shuffle(&nums, n) {
        Ok(shuffled) => println!("Shuffled: {shuffled:?}"),
        Err(e) => println!("Error: {e}"),
    }
}
