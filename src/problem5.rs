// Problem 5. Richest Customer Wealth
fn find_maxwealth(accounts: &[Vec<i32>]) -> Option<i32> {
    accounts.iter().map(|acc| acc.iter().sum()).max()
}

pub fn solve() {
    let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    match find_maxwealth(&accounts) {
        Some(val) => println!("The maximum wealth is {val}"),
        None => println!("Error: Empty input"),
    }
}
