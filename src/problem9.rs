// Problem 9. Kids With the Greatest Number of Candies
fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let max_value = *candies.iter().max().expect("No elements in the vector");
    let result: Vec<bool> = candies
        .iter()
        .map(|x| x + extra_candies >= max_value)
        .collect();
    result
}

pub fn solve() {
    let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
    let extra_candies: i32 = 3;
    let result = kids_with_candies(&candies, extra_candies);
    println!("{result:?}");
}
