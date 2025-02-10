// Problem 12. Defanging an IP Address
fn defanging(address: &str) -> String {
    address.replace('.', "[.]")
}

pub fn solve() {
    let address = "255.100.50.10";
    let result = defanging(address);
    println!("Defanging address result: {result:?}");
}
