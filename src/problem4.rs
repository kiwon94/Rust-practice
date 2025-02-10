// Problem 4. Running Sum of 1D Array
fn running_sum(v1: &[i32]) -> Vec<i32> {
    let mut v2: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for val in v1 {
        sum += *val;
        v2.push(sum);
    }
    v2
}
pub fn solve() {
    // Solution for Problem 4
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2 = running_sum(&v1);
    println!("{v2:?}");
}
