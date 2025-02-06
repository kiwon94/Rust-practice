//  Problem 2. Concatenation of Array

fn vec_concat(v1: &[i32]) -> Vec<i32> {
    let mut v2: Vec<i32> = v1.to_owned();
    v2.extend(v1.iter());
    v2
}

pub fn solve() {
    // Solution for Problem 2
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2 = vec_concat(&v1);
    println!("{v2:?}");
}
