//  Problem 1. Add Two Integers

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn solve() {
    // Solution for Problem 1
    let num1: i32 = 12;
    let num2: i32 = 5;
    let num3: i32 = add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, num3);
}
