// Problem 17 - Decode XORed Array
fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut result = vec![first];
    for &val in encoded {
        result.push(result.last().expect("Empty vector") ^ val);
    }
    result
}

pub fn solve() {
    let encoded = vec![1, 2, 3];
    let first = 1;
    let result = decode(&encoded, first);
    println!("Decoded array: {result:?}");
}
