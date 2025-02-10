// Problem 3. Build Array from Permutation
fn vec_perm(v1: &[i32]) -> Vec<i32> {
    let mut v2: Vec<i32> = Vec::new();
    for val in v1 {
        match *val {
            index
                if index >= 0
                    && index < i32::try_from(v1.len()).expect("Input format is not vector!") =>
            {
                v2.push(v1[usize::try_from(index).expect("Index out of range!")]);
            }
            _ => {
                println!("Invalid index: {val}");
            }
        }
    }
    v2
}

pub fn solve() {
    // Solution for Problem 3
    let v1: Vec<i32> = vec![0, 2, 1, 5, 3, 4];
    let v2: Vec<i32> = vec_perm(&v1);

    println!("{v2:?}");
}
