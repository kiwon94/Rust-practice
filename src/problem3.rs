// Problem 3. Build Array from Permutation
fn vec_perm(v1: &[usize]) -> Vec<usize> {
    v1.iter()
        .filter_map(|&x| {
            if x < v1.len() {
                return Some(v1[x]);
            }
            None
        })
        .collect()
}

pub fn solve() {
    // Solution for Problem 3
    let v1: Vec<usize> = vec![0, 2, 1, 5, 3, 4];
    let v2: Vec<usize> = vec_perm(&v1);

    println!("{v2:?}");
}
