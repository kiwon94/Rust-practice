// Problem 22. Final Value of Variable After Performing Operations
fn final_value_after_operations(operations: &[String]) -> i32 {
    let mut x = 0;
    for op in operations {
        match op.as_str() {
            "X++" | "++X" => x += 1,
            "X--" | "--X" => x -= 1,
            _ => (),
        }
    }
    x
}

pub fn solve() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
    let output = final_value_after_operations(&operations);
    println!("Output X: {output}");
}
