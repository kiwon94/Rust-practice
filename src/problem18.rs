// Problem 18. Decompress Run-Length Encoded List
fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut freq = 0;

    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            freq = x;
        } else {
            for _ in 0..freq {
                result.push(x);
            }
        }
    }
    result
}

pub fn solve() {
    let nums = vec![1, 2, 3, 4];
    let result = decompress_rl_elist(&nums);
    println!("Decompressed list: {result:?}");
}
