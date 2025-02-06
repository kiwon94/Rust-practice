// Problem 4. Running Sum of 1D Array

pub fn solve() {
    // Solution for Problem 4
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let mut v2: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for val in v1.iter() {
        sum += *val;
        v2.push(sum);
    }
    println!("{:?}", v2);
}
