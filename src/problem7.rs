fn minimum_sum(num: i32) -> Result<u32, String> {
    let s = num.to_string();
    let mut digits: Vec<u32> = s
        .chars()
        .map(|c| c.to_digit(10).ok_or("Invalid digit".to_string()))
        .collect::<Result<Vec<u32>, String>>()?;
    digits.sort_unstable();
    Ok(10 * (digits[0] + digits[1]) + digits[2] + digits[3])
}

pub fn solve() {
    let num = 5234;
    match minimum_sum(num) {
        Ok(sum) => println!("The minimum sum is: {sum}"),
        Err(e) => println!("Error: {e}"),
    }
}
