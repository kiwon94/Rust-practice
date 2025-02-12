// Problem 24. Create Target Array in the Given Order
fn create_target_array(nums: &[i32], index: &[i32]) -> Vec<i32> {
    let nums_len = nums.len();
    let index_len = index.len();
    assert!(
        (nums_len == index_len),
        "Nums and index must have the same length"
    );

    let mut target = Vec::new();

    for (&num, &idx) in nums.iter().zip(index.iter()) {
        let i: usize = idx.try_into().expect("Index value must be non-negative");
        assert!((i <= target.len()), "Index value is out of bounds");
        target.insert(i, num);
    }

    target
}

pub fn solve() {
    // Solution for Problem 4
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    let result = create_target_array(&nums, &index);
    println!("Target array: {result:?}");
}
