fn minimum_sum(num: i32) -> i32 {
    let num_string = num.to_string();
    let mut digits: Vec<i32> = num_string
        .chars()
        .map(|x| {
            x.to_string()
                .parse::<i32>()
                .expect("Invalid input. Must use positive integer consisting of four digits.")
        })
        .collect();
    digits.sort_unstable();

    let small1 = digits[0];
    let small2 = digits[1];
    let large1 = digits[2];
    let large2 = digits[3];

    let result: i32 = 10 * (small1 + small2) + large1 + large2;

    result
}

pub fn solve() {
    let num = 3524;
    let sum = minimum_sum(num);
    println!("The minimum sum of two numbers is: {sum}");
}
