// Problem 16. Count Items Matching a Rule
fn count_matches(items: &[Vec<String>], rule_key: &str, rule_value: &str) -> i32 {
    let idx = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => panic!("Invalid rule key: {rule_key}"),
    };
    i32::try_from(items.iter().filter(|x| x[idx] == rule_value).count())
        .expect("Failed to convert to i32")
}

pub fn solve() {
    let items = [
        vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
        vec![
            "computer".to_string(),
            "silver".to_string(),
            "lenovo".to_string(),
        ],
        vec![
            "phone".to_string(),
            "gold".to_string(),
            "iphone".to_string(),
        ],
    ];
    let rule_key = "color";
    let rule_value = "silver";
    let result = count_matches(&items, rule_key, rule_value);
    println!("Number of items matching the rule: {result}");
}
