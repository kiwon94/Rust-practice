// Problem 9. Kids With the Greatest Number of Candies
fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Option<Vec<bool>> {
    let &max_value = candies.iter().max()?;
    Some(
        candies
            .iter()
            .map(|&c| c + extra_candies >= max_value)
            .collect(),
    )
}

pub fn solve() {
    let candies = vec![1, 4, 2, 5];
    let extra_candies = 3;
    match kids_with_candies(&candies, extra_candies) {
        Some(result) => println!("The result array is : {result:?}"),
        None => println!("Error: Empty input"),
    }
}
