// Problem 24. Create Target Array in the Given Order
fn create_target_array(nums: &[i32], index: &[i32]) -> Result<Vec<i32>, String> {
    let nums_len = nums.len();
    let index_len = index.len();
    if nums_len != index_len {
        return Err("Nums and index must have the same length".to_string());
    }

    let mut target: Vec<i32> = Vec::new();

    for (&num, &idx) in nums.iter().zip(index.iter()) {
        let i: usize = match idx.try_into() {
            Ok(val) => val,
            Err(_) => return Err("Index value must be non-negative".to_string()),
        };
        if i > target.len() {
            return Err("Index value is out of bounds".to_string());
        }
        target.insert(i, num);
    }
    Ok(target)
}

pub fn solve() {
    // Solution for Problem 4
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];

    match create_target_array(&nums, &index) {
        Ok(val) => println!("Target: array: {val:?}"),
        Err(e) => println!("Error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_target_array_ok_case() {
        let nums = vec![0, 1, 2, 3, 4];
        let index = vec![0, 1, 2, 2, 1];
        let result = create_target_array(&nums, &index);
        assert_eq!(result, Ok(vec![0, 4, 1, 3, 2]));
    }

    #[test]
    fn test_create_target_array_error_length_mismatch() {
        let nums = vec![0, 1, 2, 3];
        let index = vec![0, 1, 2]; // Length mismatch
        let result = create_target_array(&nums, &index);
        assert_eq!(
            result,
            Err("Nums and index must have the same length".to_string())
        );
    }

    #[test]
    fn test_create_target_array_error_index_out_of_bounds() {
        let nums = vec![0, 1, 2, 3];
        let index = vec![0, 1, 5, 2]; // Index 5 is out of bounds
        let result = create_target_array(&nums, &index);
        assert_eq!(result, Err("Index value is out of bounds".to_string()));
    }

    #[test]
    fn test_create_target_array_error_negative_index() {
        let nums = vec![0, 1, 2];
        let index = vec![-1, 0, 1]; // Negative index
        let result = create_target_array(&nums, &index);
        assert_eq!(result, Err("Index value must be non-negative".to_string()));
    }
}
