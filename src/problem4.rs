// Problem 4. Running Sum of 1D Array
fn running_sum(v1: &[i32]) -> Vec<i32> {
    v1.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}
pub fn solve() {
    // Solution for Problem 4
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2 = running_sum(&v1);
    println!("{v2:?}");
}
