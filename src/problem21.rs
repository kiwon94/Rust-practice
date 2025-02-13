// Problem 21 - Convert Binary Number in a Linked List to Integer

fn get_decimal_value(input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, &bit| (acc << 1) | bit)
}

pub fn solve() {
    let input = vec![1, 0, 1];
    let output = get_decimal_value(&input);
    println!("Integer value: {output}");
}
