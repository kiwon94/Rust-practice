// Problem 23. Number of Steps to Reduce a Number to Zero
fn number_of_steps(num: i32) -> i32 {
    let mut num = num;
    let mut cnt = 0;

    while num != 0 {
        match num % 2 {
            0 => num /= 2,
            _ => num -= 1,
        }
        cnt += 1;
    }
    cnt
}
pub fn solve() {
    // Solution for Problem 4
    let num = 14;
    let result = number_of_steps(num);
    println!("Number of steps: {result}");
}
