// Problem 19. Shuffle String
fn restore_string(s: &str, indices: &[i32]) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut indices = indices.to_vec();
    let n = chars.len();

    for i in 0..n {
        while indices[i] != i32::try_from(i).expect("Failed to convert to i32") {
            let j = indices[i];
            chars.swap(i, usize::try_from(j).expect("Failed to convert to usize"));
            indices.swap(i, usize::try_from(j).expect("Failed to convert to usize"));
        }
    }
    chars.iter().collect()
}

pub fn solve() {
    let s = "codeleet";
    let indices = [4, 5, 6, 7, 0, 2, 1, 3];
    let result = restore_string(s, &indices);
    println!("Restored string: {result}");
}
