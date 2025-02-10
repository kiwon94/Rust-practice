// Problem 9. Kids With the Greatest Number of Candies
fn split_digits(num: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = num;

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

fn diff_product_sum(num: i32) -> i32 {
    let digits = split_digits(num);
    let product: i32 = digits.iter().product();
    let sum: i32 = digits.iter().sum();

    if sum > product {
        sum - product
    } else {
        product - sum
    }
}

pub fn solve() {
    let n = 234;
    let result = diff_product_sum(n);
    println!("Difference between sum and product is {result}");
}
