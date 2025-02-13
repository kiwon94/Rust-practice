// Problem 19. Shuffle String
use std::num::TryFromIntError;

fn restore_string(s: &str, indices: &[i32]) -> Result<String, TryFromIntError> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut indices = indices.to_vec();
    let n = chars.len();

    for idx in 0..n {
        let idx_as_i32 = i32::try_from(idx)?;
        while indices[idx] != idx_as_i32 {
            let v_idx = usize::try_from(indices[idx])?;
            chars.swap(idx, v_idx);
            indices.swap(idx, v_idx);
        }
    }

    Ok(chars.iter().collect())
}

pub fn solve() {
    let s = "codeleet";
    let indices = [4, 5, 6, 7, 1, 2, 1, 3];
    match restore_string(s, &indices) {
        Ok(result) => {
            println!("Shuffled result: {result}");
        }

        Err(e) => {
            println!("Error: {e}");
        }
    }
}
