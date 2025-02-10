// Problem 5. Richest Customer Wealth
fn find_maxwealth(accounts: &[Vec<i32>]) -> i32 {
    accounts
        .iter()
        .map(|x| x.iter().sum())
        .max()
        .expect("No elements in the vector")
}
pub fn solve() {
    let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    let max_wealth = find_maxwealth(&accounts);
    println!("The maximum wealth is {max_wealth}");
}
