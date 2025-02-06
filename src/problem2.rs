//  Problem 2. Concatenation of Array

pub fn solve() {
    // Solution for Problem 2
    let mut v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = vec![4, 5, 6];
    for i in v2.iter() {
        v1.push(*i);
    }
    println!("{:?}", v1);

    // Solution for Problem 2 (String ver.)
    // let mut v1: Vec<&str> = vec!["Hello", "World"];
    // let v2: Vec<&str> = vec!["Rust", "Programming"];
    // for s in v2.iter() {
    //     v1.push(s);
    // }
    // println!("{:?}", v1);
}
